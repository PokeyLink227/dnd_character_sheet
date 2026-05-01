## Design Goals

- As many stats/fields as possible should be auto calculated
- Support teporary statuses
    - e.g. Broken Leg -2 str
    - underwater +2 perception
- Support Items, including modifiers
    - +2 strength belt
- One click spell resource tracking
    - works for skills too
- long/short rests
- quick change equipment loadouts
- update party members with result of spells
    - send inspiration to another player and have it render a stat bonus on their sheet

- Allow for DM to manage
    - players
        - gift items
        - impose status effects
        - signal levelup
        - long/short rest
        - overide stats
            - done through the use of temporary statuses?
            - e.g. Suffocating pressence of Zura, perception is 1
    - Items
        - create items and hand them out whenever needed
    - Encounters
        - track initiave
        - create and manage monster stat blocks
    - classes/features
        - create custom classes and share with players in campaign
        - make custom spells or abilities available
            - pressence / ion stones

### Implementation Ideas

- All players/items/classes/monsters/campaigns/users will have a UUID to be referneced anywhere
- 3 tables to handle the stat dependancy tree
    - Node table
        - holds values
        - Things like Strength, Wisdom, BAB
    - Edges table
        - holds dependencies
        - e.g. Strength --> Strength Modifier
    - Calc table
        - holds calculation methods for nodes
        - e.g. Str Mod. = (Base Strength + Strength Modifiers - 10) / 2
            - (I know this is the wrong formula)

### Proposed site layout

```
Login
+-- Homepage
    +
    +-- All Character Sheets
    |   +-- View
    |   +-- Create
    +-- Campaigns
    |   +
    |   +-- Players/Characters
    |   +-- Campaign Items (DM)
    |   +-- Campaign Monsters (DM)
    |   +-- Campaign Classes (DM)
    |   +-- Campaign Notes
    +-- Items
    +-- Classes
    |   +-- View
    |   +-- Create
    +-- Notes
    |   +-- View
    |   +-- Create
    +-- Settings
```
