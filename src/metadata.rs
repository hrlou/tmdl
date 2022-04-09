pub mod prelude {
    pub use std::{fmt, str::FromStr};
    pub use futures::executor::block_on;
    pub use reqwest::{Client, Url, Method};
    pub use mlua::{Variadic, AnyUserData, UserData, MetaMethod, AsChunk};
    pub use mlua::prelude::*;
}

use prelude::*;

fn request(lua: &Lua) -> LuaResult<LuaFunction> {
    Ok(lua.create_function(|_, (method, url, body, headers): (String, String, Option<String>, Variadic<LuaTable>)| {
        let client = Client::new();
        let mut request = client.request(
            Method::from_bytes(method.as_bytes()).to_lua_err()?,
            Url::from_str(url.as_str()).to_lua_err()?
        );
        if body.is_some() {
            request = request.body(body.unwrap());
        }
        for header in headers.into_iter() {
            request = request.header(
                header.get::<_, String>(1)?,
                header.get::<_, String>(2)?
            );
        }
        let request = request.build().to_lua_err()?;
        let res = block_on(client.execute(request)).to_lua_err()?;
        let res = block_on(res.text_with_charset("UTF-8")).to_lua_err()?;
        Ok(res)
    })?)
}

fn load(lua: &Lua) -> LuaResult<()> {
    lua.globals().set("request", request(&lua)?)?;
    let json = lua.create_table()?;
    json.set("encode", lua.create_function(|_, s: LuaTable| serde_json::to_string(&s).to_lua_err())?)?;
    json.set("decode", lua.create_function(|lua, s: String| {
        let decode: serde_json::Value = serde_json::from_str(&s).to_lua_err()?;
        lua.to_value(&decode)
    })?)?;
    lua.globals().set("json", json)?;
    Ok(())
}

#[derive(Default, Clone)]
pub struct Metadata {
    pub source: String,
    pub title: String,
    pub tags: Vec<(String, String)>,
    pub files: Vec<String>,
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = 1;
        for file in &self.files {
            write!(f, "{}: {}\n", i, file)?;
            i = i + 1;
        }
        for tag in &self.tags {
            let (kind, name) = tag;
            write!(f, "{{ type: '{}', name: '{}' }}\n", kind, name)?;
        }
        write!(f, "source: {}\ntitle: {}\n", self.source, self.title)
    }
}

impl UserData for Metadata {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("source", |_, this| Ok(this.source.clone()));
        fields.add_field_method_set("source", |_, this, val| Ok(this.source = val));
        fields.add_field_method_get("title", |_, this| Ok(this.title.clone()));
        fields.add_field_method_set("title", |_, this, val| Ok(this.title = val));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("push_tag", |_, this, tag: (String, String)| Ok(this.tags.push(tag)));
        methods.add_method_mut("push_file", |_, this, file: String| Ok(this.files.push(file)));
    }
}

impl Metadata {
    pub fn new(source: String, _: Option<u8>, script: Vec<u8>) -> LuaResult<Metadata> {
        let lua = Lua::new();
        load(&lua)?;
        let mut metadata = Metadata::default();
        metadata.source = source;
        lua.globals().set("metadata", metadata)?;
        lua.load(&script).exec()?;        

        let metadata = lua.globals().get::<_, Metadata>("metadata")?;
        Ok(metadata)
    }
}