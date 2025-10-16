
# Project Todo List

- [ ] Implement enable/disable feature for menus
- [ ] engine not properly entering alt buffer
- [ ] engine not properly hiding and reveling cursor
- [ ] engine not handling different sized terminals
- [ ] need to make text box elements for world generation
- [ ] Need to make a world generation alg
- [ ] Need to test what I have with just a basic world.
    - [ ] world generation will just pop out the same very flat and small world every time with no settings.
- [ ] Will need to figure out how pagination should work for menus and such
- [ ] Watch for `SIGWINCH` signals and fire off an event to let the engine know the terminal size has changed.
- [ ] figure out the best way to link up my dependencies that I have locally with the github repo as currenlty anyone else that downloads this projects will be missing key parts of the project.
    - This doesn't work well for ui and I don't see how I can more cleanly link the rest of my program to the renderer
    - I don't really want to have to manually move al of the ui evertime I want to camera to move to a different spot in the map.
- [ ] On the rendering side implement dangling object clean up. if an arc `RenderUnitId` has no weak refs it has not connection to the update thread
- [ ] Textbox with no padding has a wierd one row of bottom padding
- [ ] Border gets pushed out on text box selection but then you can delete until the border comes back to where it should be
- [ ] input parsing doesn't handle fast typing well. seems like the terminal is sending the byte data in unexpected intervales for example sending b'do' instead of b'd' , b'o' as two different input events
- [ ] Want to be able to mix text and sprites so that I can have a mostly static text with maybe some static sprites like a text box with a blinking cursor.

## Implementations

- [ ] Selector
- [ ] State Management (What UI Element is active, what ingame menu is active if there is one)
- [ ] Basic terrian generation

