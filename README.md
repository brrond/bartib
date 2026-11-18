# Bartib

![Illustration of the White Rabbit from Alice in Wonderland](misc/white-rabbit.png "Oh dear! Oh dear! I shall be too late")

Bartib is an easy to use time tracking tool for the command line. It saves a log of all tracked activities as a plaintext file and allows you to create flexible reports.

Please see original repo for more information: https://github.com/nikolassv/bartib

## About the fork

I (@brrond) discovered this project just a couple of days ago, but I'm already a fan. However, there are some crucial functionality missing to cover use cases. As it stated by the author of Bartib, this tool was never meant to be feature rich and the project is finished. That's why I created this fork: to implement functionality I miss.

### Features of the fork

- `bartib start` without a project and/or description.
- New subcommand `bartib commit -p ... -d ...` (synonym of using `bartib change -p ... -d ... && bartib start`).
- More colors for outputs.
- Integration with tools:
  - Jira:
    - Push all entries created today.
    - Sync (WIP)
  - YouTrack (WIP)
  - GitLab (WIP)
- Auto edit of entries by `change`. (WIP)
- Configuration file. (WIP)

