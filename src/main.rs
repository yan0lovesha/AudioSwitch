mod policy_config;

use policy_config::*;
use windows::{
    core::*,
    Win32::UI::Shell::PropertiesSystem::*,
    Win32::System::Com::*,
    Win32::Media::Audio::*,
    Win32::UI::WindowsAndMessaging::*,
};

struct AudioDevice {
    id: PWSTR,
    display_name: String,
    is_default: bool
}

 fn main() -> Result<()> {
    let args_vec: Vec<String> = std::env::args().skip(1).collect();

    match (args_vec.get(0).map(|s| s.as_str()), args_vec.get(1).map(|s| s.as_str())) {
        (None, None) => list_ask(false),
        (Some("showmsg"), None) => list_ask(true),
        (Some("switch"), None) => switch_audio_device(false),
        (Some("switch"), Some("showmsg")) => switch_audio_device(true),
        _ => {
            println!("Invalid argument");
            Ok(())
        },
    }
 }

fn switch_audio_device(show_msg_box: bool) -> Result<()> {
    let devices = get_audio_device_list()?;
    let last_index = if devices.len() == 1 { 0 } else { devices.len() - 1 };
    let default_index = devices.iter().position(| device | device.is_default).unwrap();
    let target_index = if default_index == last_index { 0 } else { default_index + 1 };

    set_default_audio_device(devices[target_index].id, &devices[target_index].display_name, show_msg_box)?;

    for (index, device) in devices.iter().enumerate(){
        println!("{} {} - {}", if device.is_default {"*"} else {" "}, index, device.display_name);
    }

    Ok(())
}

fn list_ask(show_msg_box: bool) -> Result<()> {
    let devices = get_audio_device_list()?;
    for (index, device) in devices.iter().enumerate(){
        println!("{} {} - {}", if device.is_default {"*"} else {" "}, index, device.display_name);
    }

    println!("Please input the index of above devices which you want to switch to:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let selected_index: usize = input.trim().parse().unwrap();
    if selected_index < devices.len() {
        let selected_device = &devices[selected_index];
    
        set_default_audio_device(selected_device.id, &selected_device.display_name, show_msg_box)?;
    
        println!("Switched to your selected device:");
        let devices_switched = get_audio_device_list()?;
        for (index, device) in devices_switched.iter().enumerate(){
            println!("{} {} - {}", if device.is_default {"*"} else {" "}, index, device.display_name);
        }
    }
    else {
        println!("Selection out of range.");
    }

    Ok(())
}

fn get_audio_device_list() -> Result<Vec<AudioDevice>> {
    let mut device_list = Vec::new();
    unsafe {
        CoInitialize(std::ptr::null_mut())?;
        let p_enum: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let default_device = p_enum.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
        let default_device_id = default_device.GetId()?;
        let default_device_id_str = from_pwstr(default_device_id);

        let devices: IMMDeviceCollection = p_enum.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
        let count = devices.GetCount()?;
        
        for i in 0..count {
            let device = devices.Item(i)?;
            let device_id = device.GetId()?;
            let device_id_str = from_pwstr(device_id);

            let property_store: IPropertyStore = device.OpenPropertyStore(StructuredStorage::STGM_DIRECT)?;
            
            let friendly_name = property_store.GetValue(&windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName)?;
            let friendly_name_pwstr = friendly_name.Anonymous.Anonymous.Anonymous.pwszVal;

            let is_default =device_id_str == default_device_id_str; 

            device_list.push(AudioDevice{
                id: device_id,
                display_name: from_pwstr(friendly_name_pwstr),
                is_default: is_default
            });
            // println!("{}_{}",from_pwstr(device_id), from_pwstr(friendly_name_pwstr));
        }
        Ok(device_list)
    }
}

fn set_default_audio_device(device_id: PWSTR, device_name: &String, show_msg_box: bool) -> Result<()> {
    unsafe {
        CoInitialize(std::ptr::null_mut())?;
        let p_policy_config: IPolicyConfigVista = CoCreateInstance(&CPolicyConfigVistaClient, None, CLSCTX_ALL)?;
        p_policy_config.SetDefaultEndpoint(device_id, eConsole)?;
        
        if show_msg_box {
            let msg = &format!("{} {}\0", "Switched to:", &device_name)[..];
            MessageBoxA(None, ::windows::core::PCSTR::from_raw((*msg).as_ptr()), s!("Switched"), MB_OK);
        }
    }
    Ok({})
}

fn from_pwstr(p: PWSTR) -> String {
    let size = unsafe { windows::Win32::System::WindowsProgramming::uaw_wcslen(p.0) };
    let buffer = unsafe { core::slice::from_raw_parts(p.0, size) };

    // And convert from UTF-16 to Rust's native encoding
    String::from_utf16_lossy(buffer)
}


#[test]
fn test_show_msg() {
    let _ = switch_audio_device(true);
}