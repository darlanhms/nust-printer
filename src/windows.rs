use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::{ffi::CString, ptr};
use winapi::shared::{minwindef, ntdef};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winspool::{
    ClosePrinter, EndDocPrinter, EndPagePrinter, EnumPrintersW, GetDefaultPrinterW, OpenPrinterW, StartDocPrinterW, StartPagePrinter, WritePrinter, DOC_INFO_1W, PRINTER_ENUM_CONNECTIONS, PRINTER_ENUM_LOCAL, PRINTER_INFO_2W
};

use crate::printer::{Printer, PrinterState};

fn get_wchar_t_value(s: *const winapi::ctypes::wchar_t) -> String {
    if s.is_null() {
        return "".to_string();
    }

    let mut vec: Vec<u16> = Vec::new();
    let mut i = 0;
    unsafe {
        while *s.offset(i) != 0 {
            vec.push(*s.offset(i) as u16);
            i += 1;
        }
    }
    return String::from_utf16_lossy(&vec);
}

pub fn get_default_printer() -> *const winapi::ctypes::wchar_t {
    let mut name_size: winapi::ctypes::c_ulong = 0;
    unsafe {
        GetDefaultPrinterW(ptr::null_mut(), &mut name_size);
        let mut buffer: Vec<winapi::ctypes::wchar_t> = vec![0; name_size as usize];

        GetDefaultPrinterW(buffer.as_mut_ptr(), &mut name_size);
        return buffer.as_ptr();
    }
}

pub fn get_printers() -> Vec<Printer> {
    unsafe {
        let mut bytes_needed: minwindef::DWORD = 0;
        let mut num_printers: minwindef::DWORD = 0;
        let mut final_vec = Vec::<Printer>::new();

        // // Get the size needed for the buffer
        EnumPrintersW(
            0x00000002 | 0x00000004,
            ptr::null_mut(),
            2,
            ptr::null_mut(),
            0,
            &mut bytes_needed,
            &mut num_printers,
        );

        let mut printers_buffer: Vec<u8> = Vec::with_capacity(bytes_needed as usize);
        let printers_buffer_ptr = printers_buffer.as_mut_ptr();

        // Call EnumPrinters again to get the actual printer information
        EnumPrintersW(
            PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS,
            ptr::null_mut(),
            2,
            printers_buffer_ptr,
            bytes_needed,
            &mut bytes_needed,
            &mut num_printers,
        );

        // Parse the printer information
        let printer_info = printers_buffer_ptr as *mut PRINTER_INFO_2W;

        for i in 0..num_printers {
            let printer_info = *printer_info.offset(i as isize);
            let winspool_state = printer_info.Status.to_string();
            let mut state: PrinterState = PrinterState::UNKNOWN;

            if winspool_state == "0" {
                state = PrinterState::READY;
            }

            if winspool_state == "1" || winspool_state == "2" {
                state = PrinterState::PAUSED;
            }

            if winspool_state == "5" {
                state = PrinterState::PRINTING;
            }

            let printer = Printer {
                name: get_wchar_t_value(printer_info.pPrinterName),
                system_name: get_wchar_t_value(printer_info.pPrinterName),
                driver_name: get_wchar_t_value(printer_info.pDriverName),
                is_default: get_default_printer() == printer_info.pPrinterName,
                is_shared: (printer_info.Attributes & 0x00000008) == 8,
                location: get_wchar_t_value(printer_info.pLocation),
                state: state,
            };

            final_vec.push(printer);
        }

        return final_vec;
    }
}

pub fn print_direct(printer_name: String, data: &[u8]) {
    unsafe {
        // Replace "Microsoft Print to PDF" with your printer name
        let c_printer_name: Vec<u16> = OsStr::new(&printer_name)
        .encode_wide()
        .chain(once(0))
        .collect();
       
        // Open the printer
        let mut h_printer: ntdef::HANDLE = ptr::null_mut();;
    
        if {
            OpenPrinterW(
                c_printer_name.as_ptr() as *mut _,
                &mut h_printer,
                ptr::null_mut(),
            )
        } == 0
        {
            let error_code = GetLastError();
            println!("Failed to open printer. Error code: {}", error_code);
            return;
        }

        let doc_name:  Vec<u16> = OsStr::new("Test Document")
            .encode_wide()
            .chain(once(0))
            .collect();

        // Initialize the document info
        let doc_info: DOC_INFO_1W = DOC_INFO_1W {
            pDocName: doc_name.as_ptr() as *mut _,
            pOutputFile: ptr::null_mut(),
            pDatatype: ptr::null_mut(),
        };
    
        // Convert DOC_INFO_1W to *mut u8
        let doc_info_ptr: *mut u8 = mem::transmute(&doc_info);

        // Start the document
        let doc_handle =  StartDocPrinterW(h_printer, 1, doc_info_ptr);
        if doc_handle == 0 {
            let error_code =  GetLastError();
            println!("Failed to start document. Error code: {}", error_code);
            return;
        }

        // Start a new page
        if StartPagePrinter(h_printer) == 0 {
            let error_code =  GetLastError();
            println!("Failed to start page. Error code: {}", error_code);
             EndDocPrinter(h_printer) ;
            return;
        }
    
        let bytes_written = WritePrinter(
            h_printer,
            data.as_ptr() as *mut _,
            data.len() as u32,
            ptr::null_mut(),
        );

         if bytes_written == 0 {
            println!("Failed to write to printer");
        }

        // End the page and document
        if { EndPagePrinter(h_printer) } == 0 {
            println!("Failed to end page");
        }

        if { EndDocPrinter(h_printer) } == 0 {
            println!("Failed to end document");
        }

        // Close the printer handle
        if { ClosePrinter(h_printer) } == 0 {
            println!("Failed to close printer");
        }

        println!("Print succeeded then");
    }

   
}
