// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<arg::RefArg + 'static>>, Self::Err>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>, Self::Err>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<arg::RefArg>>) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusProperties for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<arg::RefArg + 'static>>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let value: arg::Variant<Box<arg::RefArg + 'static>> = i.read()?;
        Ok(value)
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"GetAll".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>> = i.read()?;
        Ok(properties)
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<arg::RefArg>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Set".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
            i.append(value);
        })?;
        m.as_result()?;
        Ok(())
    }
}

pub fn org_freedesktop_dbus_properties_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Signal: Default,
    T: OrgFreedesktopDBusProperties<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Properties", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let property_name: &str = i.read()?;
        let d = fclone(minfo);
        let value = d.get(interface_name, property_name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(value);
        Ok(vec!(rm))
    };
    let m = factory.method("Get", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.out_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let d = fclone(minfo);
        let properties = d.get_all(interface_name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(properties);
        Ok(vec!(rm))
    };
    let m = factory.method("GetAll", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.out_arg(("properties", "a{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let property_name: &str = i.read()?;
        let value: arg::Variant<Box<arg::RefArg>> = i.read()?;
        let d = fclone(minfo);
        d.set(interface_name, property_name, value)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Set", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.in_arg(("value", "v"));
    let i = i.add_m(m);
    let s = factory.signal("PropertiesChanged", Default::default());
    let s = s.arg(("interface_name", "s"));
    let s = s.arg(("changed_properties", "a{sv}"));
    let s = s.arg(("invalidated_properties", "as"));
    let i = i.add_s(s);
    i
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl dbus::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.interface_name = i.read()?;
        self.changed_properties = i.read()?;
        self.invalidated_properties = i.read()?;
        Ok(())
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    type Err;
    fn introspect(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusIntrospectable for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn introspect(&self) -> Result<String, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Introspectable".into(), &"Introspect".into(), |_| {
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let xml_data: String = i.read()?;
        Ok(xml_data)
    }
}

pub fn org_freedesktop_dbus_introspectable_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusIntrospectable<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Introspectable", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let xml_data = d.introspect()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(xml_data);
        Ok(vec!(rm))
    };
    let m = factory.method("Introspect", Default::default(), h);
    let m = m.out_arg(("xml_data", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopDBusPeer {
    type Err;
    fn ping(&self) -> Result<(), Self::Err>;
    fn get_machine_id(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusPeer for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn ping(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Peer".into(), &"Ping".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_machine_id(&self) -> Result<String, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Peer".into(), &"GetMachineId".into(), |_| {
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let machine_uuid: String = i.read()?;
        Ok(machine_uuid)
    }
}

pub fn org_freedesktop_dbus_peer_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusPeer<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Peer", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.ping()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Ping", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let machine_uuid = d.get_machine_id()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(machine_uuid);
        Ok(vec!(rm))
    };
    let m = factory.method("GetMachineId", Default::default(), h);
    let m = m.out_arg(("machine_uuid", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopNetworkManagerSettingsConnection {
    type Err;
    fn update(&self, properties: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>>) -> Result<(), Self::Err>;
    fn update_unsaved(&self, properties: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>>) -> Result<(), Self::Err>;
    fn delete(&self) -> Result<(), Self::Err>;
    fn get_settings(&self) -> Result<::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>>, Self::Err>;
    fn get_secrets(&self, setting_name: &str) -> Result<::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>>, Self::Err>;
    fn clear_secrets(&self) -> Result<(), Self::Err>;
    fn save(&self) -> Result<(), Self::Err>;
    fn update2(&self, settings: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>>, flags: u32, args: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>, Self::Err>;
    fn get_unsaved(&self) -> Result<bool, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopNetworkManagerSettingsConnection for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn update(&self, properties: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"Update".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(properties);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn update_unsaved(&self, properties: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"UpdateUnsaved".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(properties);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn delete(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"Delete".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_settings(&self) -> Result<::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"GetSettings".into(), |_| {
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let settings: ::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>> = i.read()?;
        Ok(settings)
    }

    fn get_secrets(&self, setting_name: &str) -> Result<::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"GetSecrets".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(setting_name);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let secrets: ::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>> = i.read()?;
        Ok(secrets)
    }

    fn clear_secrets(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"ClearSecrets".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn save(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"Save".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn update2(&self, settings: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>>, flags: u32, args: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.NetworkManager.Settings.Connection".into(), &"Update2".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(settings);
            i.append(flags);
            i.append(args);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let result: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>> = i.read()?;
        Ok(result)
    }

    fn get_unsaved(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings.Connection", "Unsaved")
    }
}

pub fn org_freedesktop_network_manager_settings_connection_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    D::Signal: Default,
    T: OrgFreedesktopNetworkManagerSettingsConnection<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.NetworkManager.Settings.Connection", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let properties: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>> = i.read()?;
        let d = fclone(minfo);
        d.update(properties)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Update", Default::default(), h);
    let m = m.in_arg(("properties", "a{sa{sv}}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let properties: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>> = i.read()?;
        let d = fclone(minfo);
        d.update_unsaved(properties)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("UpdateUnsaved", Default::default(), h);
    let m = m.in_arg(("properties", "a{sa{sv}}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.delete()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Delete", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let settings = d.get_settings()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(settings);
        Ok(vec!(rm))
    };
    let m = factory.method("GetSettings", Default::default(), h);
    let m = m.out_arg(("settings", "a{sa{sv}}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let setting_name: &str = i.read()?;
        let d = fclone(minfo);
        let secrets = d.get_secrets(setting_name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(secrets);
        Ok(vec!(rm))
    };
    let m = factory.method("GetSecrets", Default::default(), h);
    let m = m.in_arg(("setting_name", "s"));
    let m = m.out_arg(("secrets", "a{sa{sv}}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.clear_secrets()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("ClearSecrets", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.save()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Save", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let settings: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>> = i.read()?;
        let flags: u32 = i.read()?;
        let args: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>> = i.read()?;
        let d = fclone(minfo);
        let result = d.update2(settings, flags, args)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(result);
        Ok(vec!(rm))
    };
    let m = factory.method("Update2", Default::default(), h);
    let m = m.in_arg(("settings", "a{sa{sv}}"));
    let m = m.in_arg(("flags", "u"));
    let m = m.in_arg(("args", "a{sv}"));
    let m = m.out_arg(("result", "a{sv}"));
    let i = i.add_m(m);

    let p = factory.property::<bool, _>("Unsaved", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_unsaved()?);
        Ok(())
    });
    let i = i.add_p(p);
    let s = factory.signal("Updated", Default::default());
    let i = i.add_s(s);
    let s = factory.signal("Removed", Default::default());
    let i = i.add_s(s);
    let s = factory.signal("PropertiesChanged", Default::default());
    let s = s.arg(("properties", "a{sv}"));
    let i = i.add_s(s);
    i
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopNetworkManagerSettingsConnectionUpdated {
}

impl dbus::SignalArgs for OrgFreedesktopNetworkManagerSettingsConnectionUpdated {
    const NAME: &'static str = "Updated";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
    fn append(&self, _: &mut arg::IterAppend) {
    }
    fn get(&mut self, _: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
}

impl dbus::SignalArgs for OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    const NAME: &'static str = "Removed";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
    fn append(&self, _: &mut arg::IterAppend) {
    }
    fn get(&mut self, _: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopNetworkManagerSettingsConnectionPropertiesChanged {
    pub properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>,
}

impl dbus::SignalArgs for OrgFreedesktopNetworkManagerSettingsConnectionPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.properties = i.read()?;
        Ok(())
    }
}