function serializeTable(val, name, skipnewlines, depth)
    skipnewlines = skipnewlines or false
    depth = depth or 0
    local tmp = string.rep(" ", depth)
    if name then tmp = tmp .. name .. " = " end
    if type(val) == "table" then
        tmp = tmp .. "{" .. (not skipnewlines and "\n" or "")
        for k, v in pairs(val) do
            tmp =  tmp .. serializeTable(v, k, skipnewlines, depth + 1) .. "," .. (not skipnewlines and "\n" or "")
        end
        tmp = tmp .. string.rep(" ", depth) .. "}"
    elseif type(val) == "number" then
        tmp = tmp .. tostring(val)
    elseif type(val) == "string" then
        tmp = tmp .. string.format("%q", val)
    elseif type(val) == "boolean" then
        tmp = tmp .. (val and "true" or "false")
    else
        tmp = tmp .. "\"[inserializeable datatype:" .. type(val) .. "]\""
    end
    return tmp
end

forms = coroutine.create(function ()
  done_status = coroutine.yield({ GetOwnAttribute = { key = "done" } })
  if done_status == nil then
    return { Result =
      { done = { title = "Mark as done", label = "done",
                 action = { label = "date", title = "Day it was done", form_type = "Date" } } }
      }
  else
    return { Result = { } }
  end
end)

done = coroutine.create(function (value)
  coroutine.yield({ SysLog = "Marking note as done" })
  local date = value["Date"]
  coroutine.yield({ SetOwnAttribute = { key = "done", value = date } })
  return { Result = "Note marked as done" }
end)
