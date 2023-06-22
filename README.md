# BATTLESNAKESSSSS

## How to use this repo

This repo is to store all of unbounce's battlesnakes along with common infrastructure to deploy them. You don't have to use this repo to play battlesnake but it can be helpful to get a quick start into the game.

### Getting Started

1. Copy the folder `snakes/_template` and give it a name of your snake. The name should be URL friendly, ie. no spaces or slashes etc. This folder will contain all the source code to run your battlesnake API stack.
   1. eg: `cp -r snakes/_template snakes/my-snake-name`
2. Set snake name env var. This will make life easier later when running makefile commands
   1. `export SNAKE_NAME=my-snake-name`

3. Edit the `vars.yaml` file in your snake's folder
   1. set the snake `name` to match your snake's folder name
   2. Set `owner` to you or your team's name.
   3. Check the `port` number configured matches what your API will use
   4. Use recommended `resources` block unless you find your snake pod needs more/less

4. Edit the `Dockerfile` to work with your language stack

4. Add in your source code and get to work!

### Using the makefile

If you've exported your snake's name to the `SNAKE_NAME` var, you can use these make commands without specifying the `SNAKE_NAME=my-snake-name` before each make command.

`make build`

This will use the `Dockerfile` in your snake's folder to build and tag an image as `:snake-name-<githash>`

`make release`

This will push your tagged docker image to the shared battlesnake ECR repo so it can be used by kubernetes

`make deploy`

This will deploy your battlesnake to the kubernetes cluster so it can respond to incoming game requests. All battlesnakes share one loadbalancer but they have individual urls. URL will be
`<snake-name>-eks-integration.us-west-2.legacy.unbounce.net`

## Starting a battlesnake game

1. Head to https://play.battlesnake.com/ and create an account
2. Once logged in you can go to your profile and look for `battlesnakes`
3. Here you can register a new snake endpoint url to use for your games
4. Next hit Play Game to start a game and add your battlesnake
   1. Optionally: Add other peoples' snakes that are public using their battlesnakeUserName/battlesnake-name
5. Watch your snakes performance, tweak and redploy.

## Battlesnake API docs reference

See https://docs.battlesnake.com/ for details on how the game works and the structure of their API and objects.

## Contributing to this repo

Feel free to make any changes to files within your snake's subfolder. You don't need PR approval for anyone else, it's your space.

For shared resources in the infrastructure folder, get core to review your PR because these will effect all of the deployed battlesnakes.