-- simple sanitisation step
-- metadata is seeded by tmdl
-- 'metadata.source' is the location of the media

metadata.source = string.gsub(metadata.source, "?.*", '')
local meta = request("GET", metadata.source..'.json')
meta = json.decode(meta)

-- get tags
local fields = { "general", "character", "copyright", "artist", "meta" }
for i, field in ipairs(fields) do
    for tag in string.gmatch(meta["tag_string_"..field], "[^%s]+") do
        -- for the sake of consistancy
        if field == "copyright" then field = "parody" end
        metadata:push_tag(field, tag)
    end
end

metadata.title = "Danbooru #"..meta["id"]
metadata:push_file(meta["large_file_url"])