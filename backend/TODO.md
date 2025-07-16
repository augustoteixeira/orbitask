Work sprint
-----------

A) Editorial work
  - Delete notes
  - Functionality
    - make the root display a tree of todos?
    - layout
      - make endpoints return enum implementing Responder
      - improve css
  - Lua
    - codes: journal, article, referee
B) Monthly payments
  - Create a poll endpoint

Features
--------

  - delete note
  - build alarms
    - add alarm to sql
    - add db_function to call expired alarms (and add to all endpoints, including "/")
    - build watchdog
- Frontend
  - delete note
  - hide or change the apperence of done notes
  - breadcrumbs for ancestors
- Coding

Improvements
------------

- make an import system for one script to `requre` another in the sql database.
- make a lua library that exports functions to make it easier to call effects. For example: `log("hi")` would yield the necessary `SysLog` object.
- make execution of a note atomic from the sql perspective. We start a transaction at the start of the execution and end it later. In case there is a bug at the lua code, the execution will have no side effects.
- create a way to fix the order of the form actions. Currently it is random.
- make flash messages be a vector or json values
- infer missing capabilities automatically and suggest their inclusion with an orange flash card once a code is saved.



- implement lua effect to append a flash message
- make html pretty: https://github.com/servo/html5ever/blob/main/rcdom/examples/print-rcdom.rs
- make all endpoints return a single enum that implements Responder.
- make tests call the handler directly (instead of calling an endpoint) through Client (these tests look more like unit tests afterall).
- improve flash with a nested structure (perhaps html?) instead of a string.
- Setup login cookie expiration and implement remember-me to increate the time to 10 days (see gepeto).
- Force authentication to every call except login by hiding db in authentication (except for a simplified db for login)
- Unify treatment of errors and redirects (redirect on error is not working)
- Unify return errors to DbError (it often uses sqlx:Error)
- make ui more compact, with all information
- add logging
- make font monospaced in code editor
