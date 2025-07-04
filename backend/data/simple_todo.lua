forms = coroutine.create(function (a)
  coroutine.yield({ SysLog = "Hey!" })
  coroutine.yield({ Result =
    { crazy = { title = "Mark as done",
      action = { label = "date", title = "Day it was done", form_type = "Date" } } }
    })
end)