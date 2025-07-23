Work sprint
-----------

A) Editorial work
  - Functionality
    - eliminate root view and force the existence of a non-deletable root note, with id 0 and itself as parent.
    - make the root display a tree of todos?
  - Lua
    - codes: journal, article, referee
B) Monthly payments
  - Create a poll endpoint

Features
--------

  - build alarms
    - add alarm to sql
    - add db_function to call expired alarms (and add to all endpoints, including "/")
    - build watchdog
- Frontend
  - hide or change the apperence of done notes
  - if an attribute is long, truncate with dots: `data: JSON { date...` and show complete contents when hovering or clicking
- Coding
  - import other codes
  - make an import system for one script to `require` another in the sql database.
  - make a lua library that exports functions to make it easier to call effects. For example: `log("hi")` would yield the necessary `SysLog` object.
  - add `external` boolean column to the code table
  - create table for remotes
  - add functionality to import remote code
  - add `display` functionality: an endpoint from lua that returns html to be displayed below the note's description. this could replace the subnotes and the tree of subnotes.

Improvements
------------

- implement a new column (data_types) for logs and in each log create a link to view the log with a visualizer for common types like json, text, images... then add a lot of data to each auto log.
- create a way to fix the order of the form actions. Currently it is random.
- make flash messages be a vector or json values
- infer missing capabilities automatically and suggest their inclusion with an orange flash card once a code is saved.



- implement lua effect to append a flash message
- make tests call the handler directly (instead of calling an endpoint) through Client (these tests look more like unit tests afterall).
- improve flash with a nested structure (perhaps html with maud?) instead of a string.
- Setup login cookie expiration and implement remember-me to increate the time to 10 days (see gepeto).
- Force authentication to every call except login by hiding the access to the database in authentication (except for a simplified db used exclusevely for login)
- Unify treatment of errors and redirects
- Unify return errors to DbError (it often uses sqlx:Error)
- add logging
