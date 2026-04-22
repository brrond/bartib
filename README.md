# Bartib

![Illustration of the White Rabbit from Alice in Wonderland](misc/white-rabbit.png "Oh dear! Oh dear! I shall be too late")

Bartib is an easy to use time tracking tool for the command line. It saves a log of all tracked activities as a plaintext file and allows you to create flexible reports.

Please see original repo for more information: https://github.com/nikolassv/bartib

## Contents

- [Bartib](#bartib)
  - [Contents](#contents)
  - [About the fork](#about-the-fork)
    - [Features of the fork](#features-of-the-fork)

## About the fork

I (@brrond) discovered this project just a couple of days ago, but I'm already a fun. However, there are some crucial functionality missing to cover my use cases. As it stated by the author of Bartib, this tool was never meant to be feature rich and the project is finished. That's why I created this fork: to implement functionality I miss.

### Features of the fork

- `bartib start` without a project and/or description ([#1](https://github.com/brrond/bartib/pull/1)).
  
    Use case: I get a call from a college and want to stop the current task and start the next one, but I don't know what it will be all about. I don't have project and/or description yet.
    Now I can simply use `bartib start` and later `bartib change -p ... -d ...`.

- New subcommand `bartib commit -p ... -d ...` ([#1](https://github.com/brrond/bartib/pull/1)).
  
    Synonym of using `bartib change -p ... -d ... && bartib start`

- More colors for outputs ([#2](https://github.com/brrond/bartib/pull/2)).

- Integration with tools:
  - Jira:
    - Push all today entries ([#3](https://github.com/brrond/bartib/pull/3)).

      Pushes all entires to Jira, if JIRA_SERVER, JIRA_USER and JIRA_TOKEN are specified via env.

    - Commit & push (WIP)
    - Sync (WIP)
   
  - YouTrack (WIP)

- Auto edit of entries by `change` (WIP)
