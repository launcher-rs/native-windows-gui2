use crate::{Bitmap, ImageData, NwgError};
use std::ptr;
use winapi::Interface;
use winapi::ctypes::{c_uint, c_void};
use winapi::shared::winerror::S_OK;
use winapi::um::objidlbase::IStream;
use winapi::um::wincodec::{IWICBitmapDecoder, IWICImagingFactory};

pub fn create_image_factory() -> Result<*mut IWICImagingFactory, NwgError> {
    use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
    use winapi::um::combaseapi::CoCreateInstance;
    use winapi::um::wincodec::CLSID_WICImagingFactory;

    let mut image_factory: *mut IWICImagingFactory = ptr::null_mut();
    let result = unsafe {
        CoCreateInstance(
            &CLSID_WICImagingFactory,
            ptr::null_mut(),
            CLSCTX_INPROC_SERVER,
            &IWICImagingFactory::uuidof(),
            (&mut image_factory as *mut *mut IWICImagingFactory) as *mut *mut c_void,
        )
    };

    if result != S_OK {
        return Err(NwgError::resource_create(
            "Failed to create a image factory",
        ));
    }

    Ok(image_factory)
}

pub fn create_decoder_from_file<'a>(
    fact: &IWICImagingFactory,
    path: &'a str,
) -> Result<*mut IWICBitmapDecoder, NwgError> {
    use crate::win32::base_helper::to_utf16;
    use winapi::um::wincodec::WICDecodeMetadataCacheOnDemand;
    use winapi::um::winnt::GENERIC_READ;

    let path = to_utf16(path);

    let mut decoder: *mut IWICBitmapDecoder = ptr::null_mut();
    let result = unsafe {
        fact.CreateDecoderFromFilename(
            path.as_ptr(),
            ptr::null(),
            GENERIC_READ,
            WICDecodeMetadataCacheOnDemand,
            (&mut decoder as *mut *mut IWICBitmapDecoder) as *mut *mut IWICBitmapDecoder,
        )
    };

    if result != S_OK {
        return Err(NwgError::resource_create(
            "Failed to create a bitmap decoder",
        ));
    }

    Ok(decoder)
}

// This function is not declared by winapi yet. But winapi have import libraries
// for shlwapi (when the shellapi feature is enabled), so we can use it if we
// declare it ourselves.
unsafe extern "system" {
    fn SHCreateMemStream(p_init: *const u8, cb_init: c_uint) -> *mut IStream;
}

pub fn create_decoder_from_stream(
    fact: &IWICImagingFactory,
    data: &[u8],
) -> Result<*mut IWICBitmapDecoder, NwgError> {
    use std::convert::TryInto;
    use winapi::um::wincodec::WICDecodeMetadataCacheOnDemand;

    let stream = unsafe {
        SHCreateMemStream(
            data.as_ptr(),
            data.len().try_into().map_err(|_| {
                NwgError::resource_create("Failed to create memory stream, stream is too long")
            })?,
        )
    };
    if stream.is_null() {
        return Err(NwgError::resource_create(
            "Failed to create memory stream, allocation failure",
        ));
    }

    let mut decoder: *mut IWICBitmapDecoder = ptr::null_mut();
    let r = unsafe {
        fact.CreateDecoderFromStream(
            stream,
            ptr::null(),
            WICDecodeMetadataCacheOnDemand,
            (&mut decoder as *mut *mut IWICBitmapDecoder) as *mut *mut IWICBitmapDecoder,
        )
    };

    unsafe {
        (*stream).Release();
    }

    if r != S_OK {
        return Err(NwgError::resource_create(
            "Failed to create decoder from stream",
        ));
    }

    Ok(decoder)
}

pub fn create_bitmap_from_wic(image: &ImageData) -> Result<Bitmap, NwgError> {
    use std::mem;
    use winapi::shared::{minwindef::DWORD, ntdef::LONG, windef::HBITMAP};
    use winapi::um::wincodec::{
        GUID_WICPixelFormat32bppPBGRA, IWICBitmapSource, WICConvertBitmapSource,
    };
    use winapi::um::wingdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateDIBSection, DIB_RGB_COLORS, DeleteObject,
        RGBQUAD,
    };
    use winapi::um::winuser::{GetDC, ReleaseDC};

    // First convert the image into a bitmap compatible format
    let frame_ptr = unsafe { (&*image.frame) as &IWICBitmapSource as *const IWICBitmapSource };
    let mut converted = ptr::null_mut();
    let hr = unsafe {
        WICConvertBitmapSource(&GUID_WICPixelFormat32bppPBGRA, frame_ptr, &mut converted)
    };

    if hr != S_OK {
        return Err(NwgError::image_decoder(
            hr,
            "Could not convert image pixels",
        ));
    }

    // Converted size
    let (mut width, mut height) = (0, 0);
    unsafe {
        (&*converted).GetSize(&mut width, &mut height);
    }

    // Prepare the bitmap
    let header = BITMAPINFOHEADER {
        biSize: mem::size_of::<BITMAPINFOHEADER>() as DWORD,
        biWidth: width as LONG,
        biHeight: -(height as LONG),
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB,
        biSizeImage: (width * height * 3) as u32,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    let quad = RGBQUAD {
        rgbBlue: 0,
        rgbGreen: 0,
        rgbRed: 0,
        rgbReserved: 0,
    };
    let bitmap_info = BITMAPINFO {
        bmiHeader: header,
        bmiColors: [quad],
    };

    // Create a DIB
    let mut bits = ptr::null_mut();
    let screen_dc = unsafe { GetDC(ptr::null_mut()) };
    let bitmap = unsafe {
        CreateDIBSection(
            screen_dc,
            &bitmap_info,
            DIB_RGB_COLORS,
            &mut bits,
            ptr::null_mut(),
            0,
        )
    } as HBITMAP;
    unsafe {
        ReleaseDC(ptr::null_mut(), screen_dc);
    }

    if bitmap.is_null() {
        return Err(NwgError::image_decoder(hr, "Could not create a bitmap"));
    }

    // Write the DIB data
    let stride = width * 4;
    let bitmap_size = width * height * 4;
    let hr = unsafe { (&*converted).CopyPixels(ptr::null(), stride, bitmap_size, bits as *mut u8) };
    if hr != S_OK {
        unsafe {
            DeleteObject(bitmap as _);
        }
        return Err(NwgError::image_decoder(hr, "Could not write to bitmap"));
    }

    // Free up the converted image
    unsafe {
        (&*converted).Release();
    }

    Ok(Bitmap {
        handle: bitmap as _,
        owned: true,
    })
}

pub fn resize_bitmap(
    fact: &IWICImagingFactory,
    image: &ImageData,
    new_size: [u32; 2],
) -> Result<ImageData, NwgError> {
    use winapi::um::wincodec::{
        IWICBitmapScaler, IWICBitmapSource, WICBitmapInterpolationModeCubic,
    };

    let mut scaler: *mut IWICBitmapScaler = ptr::null_mut();
    let result = unsafe { fact.CreateBitmapScaler(&mut scaler) };
    if result != S_OK {
        return Err(NwgError::image_decoder(
            result,
            "Could not create bitmap scaler",
        ));
    }

    let [w, h] = new_size;
    let image_source = image.frame as *const IWICBitmapSource;
    let result =
        unsafe { (&*scaler).Initialize(image_source, w, h, WICBitmapInterpolationModeCubic) };
    if result != S_OK {
        return Err(NwgError::image_decoder(
            result,
            "Could not initialize bitmap scaler",
        ));
    }

    Ok(ImageData {
        frame: scaler as *mut IWICBitmapSource,
    })
}
