# Plan:


### Version 0.8.0
- [ ] Notebook Support
  - [ ] Separates different notes into notebooks
  - [ ] Enable/Disable for viewing
  - [ ] See notes from different bible versions
- [ ] Note viewer
  - [ ] Can see all notes without having to find them
  - [ ] Searching based on keywords?
  - [ ] View multiple notebooks

### Version 0.7.0: Multi-Platform Support
- [ ] Android
- [ ] MAC
- [ ] IOS
- [ ] Linux?

### Version 0.6.0: Lexicon Support
- [ ] Strong's
- [ ] Brown's Driver's Brig's
- [ ] Treasury of Scriptural Knowledge
- [ ] Lexicon inspector
  - [ ] Collapsible
  - [ ] On side menu
  - [ ] Looks nice
- [ ] Collapse long notes/highlights
- [ ] Strong's number parsing/searching
- [ ] Support relegated to versions of the Bible where compatible
- [ ] Bugs
  - [ ] CRITICAL - 3rd John does not appear, it is just called John

### Version 0.5.4: Cloud Saving
- [x] Cloud saving via Google Drive
- [ ] ~~Auto saving (at intervals)~~
  - [ ] ~~Auto syncing?~~
- [x] Bugfixes
  - [x] Scrollbar appears when it does not need to on erase highlight dropdown when reversed
  - [x] Crashes on June 19th Robert Roberts reading, problem parsing 2 John-3 John
  - [x] Highlight/Note menu not clamped horizontally, and possibly vertically
  - [x] Deleting reference button does not work properly
  - [x] Audio player auto plays whenever the user changes the behavior
- [x] Added footer to all pages
- [x] When erasing all highlights, a confirm popup will be displayed if selecting over 20 words

### Version 0.5.3: Bug Fixes and Improvements
- [x] Lots o Bugs fixes
  - [x] Removed close image on search note page
  - [x] Pointer curser now on chapter buttons
  - [x] Invalid display when creating an empty note
  - [x] Edit note button changed to make like rest
  - [x] Reference buttons on notes and section buttons on notes now have curser pointer
  - [x] Reference buttons on notes and the edit note button now have proper titles
  - [x] Note not saving when moving to a different page (ie: next chapter)
  - [x] Long text not wrapping via `-` In the note viewer
  - [x] Verse quotations in notes can be a tad too long
  - [x] Excluded verses aren't handled
  - [x] Unable to search for `song of solomon` (bug introduced in this update)
  - [x] Audio player playback slider finishing on 0 instead of 1
  - [x] References are now clickable while in the text editor
  - [x] Section note page did not have the extra spacing after the chapter content like its non-note counterpart
  - [x] Word popup stopping interactions with selection menu
  - [x] Highlight menu not restricted to inside of page
  - [x] Audio Player not clamped to inside of page
  - [x] When selecting words, if a space was selected after a given word, it would select the next word as well (also fixed a hidden rendering bug as well)
  - [x] Fixed visuals when closing a note on faster computers (now has an animated collapse)
  - [x] Highlighting and opening the context menu, then clicking on the text or its parent again, removes the curser highlights, but leaves the context menu
- [x] Finalize audio playback
- [x] Better highlighting
  - [x] No highlights created
  - [x] Better highlight editor
    - [x] Now with WYSIWYG
    - [x] Looks nicer
    - [x] Change highlight viewer layout
  - [x] Preset highlights (when none created)
  - [x] Highlight selector (recents)
  - [x] Better selector system
    - [x] Improve look
    - [x] Erase all highlight button
    - [x] Create new highlight button
  - [x] Can now edit highlight in sidebar menu
- [x] Miscellanies
  - [x] Words with notes or highlights now have `pointer` curser when hovered
  - [x] When closing editing note, scroll to where edited it
  - [x] When editing an already created note, scroll to where editing it
  - [x] Scroll offset to center, not top
  - [x] Migrate save settings
    - [x] Audio player
    - [x] Recent highlights
  - [x] Added a Footer to Bible pages (apparently they were looking a bit bare)
  - [x] Resizer bar now shows if a side page is collapsed
  - [x] Rename collapse buttons to be clearer
  - [x] Header Ui Refactor
    - [x] Dropdowns can now be toggled to active
    - [x] Backend fixed to make the code less reliant on ids
    - [x] Some visual upgrades
  - [x] Added tooltip for chapter links on the daily readings page
  - [x] Changed ul/ol margins for notes and highlight descriptions
  - [x] Side info panel's min and max size is now dependent on the window min and max size


### Version 0.5.2: Note QoL Changes
- [x] Selecting Notes and Highlights
  - [x] Single select and use
  - [x] When select, dropdown to create note, add to note, highlight, or erase
- [x] WYSIWYG Note Editor
  - [x] Locations
    - [ ] ~~On the Highlight Description~~
    - [x] On the Note Editor
  - [x] Features
    - [x] Basic controls: Bold, Italics, Lists, etc
    - [ ] ~~Syntax highlighting?~~
- [x] Better note references section
- [x] Miscellaneous
  - [x] Changed button scale again
  - [x] Added title to the audio player button
  - [x] Fixed radius on text dropdown
  - [x] Fixed search button hover effects
  - [x] Fixed look of chapter buttons
  - [x] Fixed cursor when hovering on edit note button
  - [x] Drop shadows now properly applied to side panels

### Version 0.5.1: Text to Speech
- [x] Text to speech
  - [x] Change speed
  - [x] Change volume
  - [ ] ~~Auto play~~
    - [ ] ~~Time based~~
    - [ ] ~~Chapter number based~~
    - [ ] ~~Readings based~~
  - [ ] ~~Voice selector~~
  - [x] Skip
  - [x] Replay
  - [x] Player visual
- [ ] ~~Multiple Theme Support~~
  - [ ] ~~Dark~~
  - [ ] ~~Light~~
  - [ ] ~~Parchment~~
- [x] Create better info page
- [ ] ~~Make calendar selected date more visible~~

### Version 0.5.0: Multiple Bible Support
- [x] Multiple Bible Support
  - [x] A notebook for each bible (for now)
  - [x] Can swap between bibles
  - [x] Can search in different bibles
  - [x] Conserve scroll when swapping
- [x] Readings work with the different bibles (select which one)
- [x] Splash Screen (to hide loading)
- [x] Bugfixes
  - [x] Volume slider works properly again
  - [x] Fixed crash with going past last chapter
- [x] Proper migration
- [x] When creating new note, scroll into view
- [x] When searching for verses, scroll into view
- [x] Fix close side popup in search view

### Version 0.4.5: Adding Reading Plans
- [x] Adding built in reading plans
- [x] Also did other stuff

### Version 0.4.4: Hotfix
- [x] Fixed a bug with the volume slider

### Version 0.4.3: Visualization Overall
- [x] Implementing LESS
- [x] Settings
  - [x] Different Font Support
    - [x] Arial (Default)
    - [x] Times New Roman
    - [x] Brush Script
    - [x] Courier New
  - [x] Text Size
  - [x] UI Size
  - [x] Volume
- [x] Min Window Size
- [x] Sounds
  - [x] Page flipping
- [x] More tooltips
- [x] Tutorial Page (WIP)

### Version 0.4.2: Bugfixes
- [x] Bugfixes:
  - [ ] ~~Crash with note creation~~
  - [x] Context menu:
    - [x] Erase highlights with none created
    - [x] Set highlight with none created
    - [x] Context menu highlighting selection not updating highlight dropdown preview 
  - [x] Note visual not showing properly in first verse
  - [x] Fix the confirm delete note popup message
  - [x] Fix the confirm delete highlight popup message
- [x] Updates:
  - [x] Save files with versioning
  - [x] Visual for selecting notes
  - [x] Visual when save file not found
  - [x] Add `Jas` shorthand for James
  - [x] Update icon
- [x] Misc
  - [x] Fix spelling of `categories`
  - [x] ON BUILD: change file from short_kjv.txt to kjv.txt


### Version 0.4.1: Various Bugfixes
- [x] Note reference location view fix
- [x] Context menu interfering with spellchecker
- [x] Context menu positioning issue
- [x] Empty note looks weird
- [x] Delete note button in weird location
- [x] Toolbar for the editor not collapsing properly

### Version 0.4.0: Proper Note Taking
- [x] Verse notes
  - [x] Cross references
  - [x] Editor
  - [x] Links to other sections in scripture
- [ ] ~~Favorite verses~~
- [ ] ~~Section headings~~
- [x] Viewer overhaul
  - [x] Make viewer look nicer
  - [x] Allow for quick edits
  - [x] Viewer for word and verses

### Version 0.3.2: General Fixes
- [x] More text alternates for buttons
- [x] Bugs:
  - [x] Searching for spaces is allowed
  - [x] Punctuation in highlight names
  - [x] Reset scroll
  - [x] Duplicate words in searches break
- [x] Chapter History
- [x] Better section display for large word searches
  - [x] Make buttons look better
  - [x] Show chapter sections (ei: 1 Sam 4:4 -  Job 32:7), etc
  - [x] Need way to display short chapter names (first four letters?)
- [x] Can check for multiple of teh same words
  - [x] Will check frequency of those words
- [x] Top bar
  - [x] Make bigger
  - [x] Make buttons not shrink when make window smaller
- [x] Figure out name
- [x] Allow copy paste text

### Version 0.3.1: Hotfix 
- [x] Bug with large number of searched word verses
- [x] Word search large section selection

### Version 0.3.0: Searching
- [x] Improve Bible Viewer
  - [x] Better book/chapter selection
  - [x] Next/previous chapter buttons
  - [x] Search bar
- [x] Search-bar
  - [x] Word searches
    - [x] Returns result of all verses with words
    - [ ] ~~Has a priority system~~
    - [ ] ~~Closer to phrases better~~
  - [x] Section searches
    - [x] Parses searches similar to blue letter bible
    - [x] Goes to first verse when verse section selected
      - [ ] ~~Only display selected verses???~~

### Version 0.2.0: Highlight Support
- [x] Highlight Category:
  - [x] Data:
    - [x] Color
    - [x] Basic information on what its about
    - [x] Name
    - [x] Priority
  - [x] Can create new categories
  - [x] Can remove old categories
    - [x] Will have a warning message if any are in use
- [x] Highlighting Scripture
  - [x] Highlight Selection
    - [x] Select category
    - [x] Select area
  - [x] Removing Highlights
    - [x] Select category
    - [x] Hover over verses to remove
- [x] Save and load highlights
  - [x] Test clearing highlights
- [x] Hover display description
  - [x] Displays little popup with name when hovered over
  - [x] When clicked, creates sidebar with all names and description of highlights

### Version 0.1.0: Get Things Working
- [x] Display KJV bible
- [x] Chapter Selection
- [x] Looks decent
