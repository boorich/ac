# ac
A tool to fetch programming languages from Github.
By default, it fetches the most popular languages.
Beware this is a work in progress.

# Usage
Add constants here:

    // some constants for fetching different repo's data
        ...
        const URL3: &str = "https://api.github.com/repos/empea-careercriminal/concierge/languages";

Call the function here:

    response = client
        ...
        .get(URL3)
        ...
