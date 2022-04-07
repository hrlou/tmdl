mod lua {
    use std::path::Path;
    use futures::executor::block_on;
    use mlua::{Variadic, AnyUserData};
    use mlua::prelude::*;
    
    fn json(lua: &Lua) -> LuaResult<LuaTable> {
        let exports = lua.create_table()?;
        exports.set("decode", lua.create_function(|lua, s: String| {
            let decode: serde_json::Value = serde_json::from_str(&s).to_lua_err()?;
            lua.to_value(&decode)
        })?)?;
        exports.set("encode", lua.create_function(|_, s: LuaValue| {
            serde_json::to_string(&s).to_lua_err()
        })?)?;
        Ok(exports)
    }

    fn web(lua: &Lua) -> LuaResult<LuaTable> {
        use reqwest::Client;
        let exports = lua.create_table()?;
        exports.set("get", lua.create_function(|_, url: String| {
            let client = Client::new();
            let response = block_on(client.get(url).send()).to_lua_err()?;
            let response = block_on(response.text()).to_lua_err()?;
            Ok(response)
        })?)?;
        exports.set("post", lua.create_function(|_, (url, kind, body): (String, String, String) | {
            let client = Client::new();
            let request = client.post(url)
                .header("Content-Type", kind)
                .body(body);
            let response = block_on(request.send()).to_lua_err()?;
            let response = block_on(response.text()).to_lua_err()?;
            Ok(response)
        })?)?;
        Ok(exports)
    }

    fn tmdl(lua: &Lua) -> LuaResult<LuaTable> {
        let exports = lua.create_table()?;
        exports.set("push_tag", lua.create_function(|_, (field, name): (String, String)| {
            println!("type:{}\tname:{}", field, name);
            Ok(())
        })?)?;
        // exports.set("push_file", )
        Ok(exports)
    }

    pub fn download<P: AsRef<Path>, T: Into<String>>(script: P, url: T) -> LuaResult<()> {
        let lua = Lua::new();
        let f = std::fs::read(script)?;
        lua.globals().set("URL", url.into())?;
        lua.globals().set("tmdl", tmdl(&lua)?)?;
        lua.globals().set("json", json(&lua)?)?;
        lua.globals().set("web", web(&lua)?)?;
        lua.globals().set("printf", lua.create_function(|lua, v: Variadic<AnyUserData>| {
            let string: LuaTable = lua.globals().get("string")?;
            let format: LuaFunction = string.get("format")?;
            let s = format.call::<_, String>(v)?;
            print!("{}", s);
            Ok(())
        })?)?;
        lua.load(&f).exec()?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}