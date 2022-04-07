-- URL is seeded by tmdl
-- simple sanitisation step
URL = URL:gsub("?.*", '')
local meta = URL..'.json'
meta = web.get(meta)
meta = json.decode(meta)

-- get tags
local fields = { "general", "character", "copyright", "artist", "meta" }
for i, field in ipairs(fields) do
    for tag in string.gmatch(meta["tag_string_"..field], "[^%s]+") do
        -- for the sake of consistancy
        if field == "copyright" then field = "parody" end
        tmdl.push_tag(field, tag)
    end
end

tmdl.push_tag("title", "Danbooru #"..meta["id"])
-- print("ID", meta["id"])
-- print("PUSH_FILE", meta["large_file_url"])