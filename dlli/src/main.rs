mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Data::Xml::Dom::*,
    Windows::Win32::Foundation::CloseHandle,
    Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
    Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
};

fn main() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, "Hello, world!!!", "Caption", MB_OK);
    }

    Ok(())
}

