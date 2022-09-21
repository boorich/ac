# ac
A tool to fetch programming languages from Github.

# Usage
Add constants here:

    // some constants for fetching different repo's data
        ...
        const URL3: &str = "https://api.github.com/repos/empea-careercriminal/concierge/languages";

Call the function here:

    response = client
        .expect("Didn´t work")
        .get(URL3)
        ...
