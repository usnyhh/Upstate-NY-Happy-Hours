# Database design and use case

## user table


##### owner table
- Id: a unique id to each user
- Username: the username of the user
- Email: the email of the user
- password: the password of the user

##### The user table consists of
- Id: a unique id to each user
- Username: the username of the user
- Email: the email of the user
- password: the password of the user
- favorates: favorate locations by id

##### the location table consists of
- ID: unique id of the given location
- Address: The address of the location
- open_time: the opening time of the location
- close_time: the closing time of the location
- happy_hour_times: the time period in which the resturant is running a happy hour
- menu_items: point towards a set of menu items
- owner_verified: ether null or the owner account accociated with the table

##### review item
ID: unique id of the given review
location_id: the id of the given location
Stars: the star review number of the review
content: the writen content of the review

##### menu item
- ID: unique id of the given menu item
- reterant_id: the id of the resterant the menu item is served at
- is_discounted: if the item has a happy hour discount
- Undiscounted: The price of the menu item before the happy hour discount
- Discounted 
- Deal: a misc extra deal that isnt a price decrease
- owner_verification: null or owner