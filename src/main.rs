use zbus::{Connection, fdo::PropertiesProxy, names::InterfaceName, zvariant::OwnedValue};

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::system().await?;

    let upower_device_interface = InterfaceName::from_static_str("org.freedesktop.UPower.Device")?;

    let proxy = PropertiesProxy::builder(&connection)
        .destination("org.freedesktop.UPower")?
        .path("/org/freedesktop/UPower/devices/battery_devxbatteryx1")?
        .build()
        .await?;

    let state_raw: OwnedValue = proxy.get(upower_device_interface.clone(), "State").await?;
    let state = BatteryState::new(state_raw.downcast_ref::<u32>().unwrap_or(&10));
    // let state_str = match state {
    //     1 => "charging",
    //     2 => "discharging",
    //     3 => "empty",
    //     4 => "fully charged",
    //     5 => "pending charge",
    //     6 => "pending discharge",
    //     _ => "unknown",
    // };

    let percentage_raw: OwnedValue = proxy.get(upower_device_interface, "Percentage").await?;
    let percentage = percentage_raw.downcast_ref::<f64>().unwrap_or(&0.0);

    // Print battery info
    println!("Battery State: {}", state);
    println!("Battery Percentage: {:.1}%", percentage);

    Ok(())
}

enum BatteryState {
    Charging,
    Discharging,
    Empty,
    FullyCharged,
    PendingCharge,
    PendingDischarge,
    Unknown,
}

impl BatteryState {
    fn new(state: &u32) -> Self {
        BatteryState::from(*state)
    }
}

impl From<u32> for BatteryState {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Charging,
            2 => Self::Discharging,
            3 => Self::Empty,
            4 => Self::FullyCharged,
            5 => Self::PendingCharge,
            6 => Self::PendingDischarge,
            _ => Self::Unknown,
        }
    }
}

impl std::fmt::Display for BatteryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BatteryState::Charging => write!(f, "{}", "charging"),
            BatteryState::Discharging => write!(f, "{}", "discharging"),
            BatteryState::Empty => write!(f, "{}", "empty"),
            BatteryState::FullyCharged => write!(f, "{}", "fully charged"),
            BatteryState::PendingCharge => write!(f, "{}", "pending charge"),
            BatteryState::PendingDischarge => write!(f, "{}", "pending discharge"),
            BatteryState::Unknown => write!(f, "{}", "unknown"),
        }
    }
}
