local base, id = string.match(metadata.source, "https?://([%w%p]+)/g/(%d+)")

local meta = "https://"..base.."/api/gallery/"..id
meta = request("GET", meta)
meta = json.decode(meta)

IMAGE_URL = "https://i."..base.."/galleries/"..meta["media_id"].."/"
EXTS = { ["j"] = "jpg", ["p"] = "png", ["g"] = "gif" }

for i, tag in ipairs(meta["tags"]) do
    if tag["type"] == "tag" then tag["type"] = "general" end
    metadata:push_tag(tag["type"], tag["name"])
end

for i, page in ipairs(meta["images"]["pages"]) do
    metadata:push_file(IMAGE_URL..i.."."..EXTS[page["t"]])
end

metadata.title = meta["title"]["pretty"]