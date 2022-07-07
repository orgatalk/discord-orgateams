# OrgaTalk Discord Orga Teams

List orga teams on the [OrgaTalk](https://www.orgatalk.de/) community's
[Discord](https://discord.com/) server.

The OrgaTalk Discord community uses roles to tag members with the LAN
party organizer teams they are part of. This tool exports those to a
static HTML file for display on the web.

Since the role assignments change over time, it makes sense to regularly
run the tool to update the HTML export. For this purpose, a
[cronjob](https://en.wikipedia.org/wiki/Cron) is commonly used.


## Usage

To get an overview of the available options, use the `--help` option:

```sh
$ orgatalk-discord-orgateams --help
```


### Configuration File

A configuration file is required. Create a custom one based off of the
included [`config-example.toml`](config-example.toml). It uses the
[TOML](https://toml.io/) format.

The `roles_excluded` property expects a list of role names to exclude
from the output. This can be roles only given to bots or, in the case of
the OrgaTalk community, anything that does not represent an organizer
role.

Then run the program, specifying it:

```sh
$ orgatalk-discord-orgateams --config=your-config.toml
```


### Output

The output will be written to standard output (stdout). To write the
output to a file, specify the `--output` option:

```sh
$ orgatalk-discord-orgateams --config=your-config.toml --output=output.html
```

By default, the output format is HTML. Alternatively, it can be plain
text …

```sh
$ orgatalk-discord-orgateams --config=your-config.toml --format=text
```

… or JSON:

```sh
$ orgatalk-discord-orgateams --config=your-config.toml --format=json
```


### Import Roles From File

Instead of fetching roles from the Discord API on every invocation, they
can be read from a local JSON file:

```sh
$ orgatalk-discord-orgateams --config=your-config.toml --roles-input=roles.json
```

That expects the roles to first be exported like this:

```sh
$ orgatalk-discord-orgateams --config=your-config.toml --format=json --output=roles.json
```

This avoids unnecessary requests to the Discord API and, thus, speeds up
the program. It is helpful during development and customization of the
HTML and text templates.


## License

This program is licensed under the MIT license.


## Author

Jochen Kupperschmidt
