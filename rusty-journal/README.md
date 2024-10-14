## Journal App

The application is a command-line to-do tracker. It records our tasks into a text file, displays them as a list in the terminal, and lets us mark them complete.

The program will persist our to-do items in some kind of storage. A text file should be good enough to store this kind of data, so we can stick to a file format like JSON to encode our information. We'll need to handle saving data to storage and retrieving it from storage.

Now that we've specified our application's use cases, we can allocate each action into its own module. It would make sense to have modules for command-line parsing and task persistence and then use the main.rs module to link them together and handle all possible errors.

Because we'll manipulate to-do tasks, we should also have a Task struct to keep track of each to-do item.

Having said that, let's create our initial project template. In your local development environment, create a new Cargo project by using the cargo new command in your terminal. Call the project rusty-journal.

### Build

`$ cargo build --profile release-lto`

The output for each profile will be placed in a directory of the same name as the profile in the **target directory**. As in the example above, the output would go into the `target/release-lto` directory.

### Run

`$ cargo run -- -j test-journal.json add "buy milk"`

`$ cargo run -- -j test-journal.json add "take the dog for a walk"`

`$ cargo run -- -j test-journal.json add "water the plants"`

`$ cargo run -- -j test-journal.json list`
>> 1: buy milk                                           [2021-01-08 16:39]<br>
>> 2: take the dog for a walk                            [2021-01-08 16:39]<br>
>> 3: water the plants                                   [2021-01-08 16:39]

`$ cargo run -- -j test-journal.json done 2`

`$ cargo run -- -j test-journal.json list`
>> 1: buy milk                                           [2021-01-08 16:39]<br>
>> 2: take the dog for a walk                            [2021-01-08 16:39]<br>