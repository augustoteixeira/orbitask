forms = coroutine.create(function ()
    return { Result =
      { create = { title = "Create a child", label = "create",
                   action = { label = "number", title = "Number of child",
                              form_type = "UInt" } } } }
end)

create = coroutine.create(function (value)
  id = coroutine.yield("GetId")
  coroutine.yield({ SysLog = "Creating child" })
  local number = value["UInt"]
  child_id = coroutine.yield({ CreateChild = {
    parent_id = id, title = "Number: " .. tostring(number),
    description = "This is cool!", code_name = nil } })
  coroutine.yield({ SysLog = tostring(child_id) })
  return { Result = "Child created" }
end)
