# Prompter

This program is meant to generate the contents of the psvar array so that zsh prompts
can then use them. In addition it can output the prompt variable content and other
initialisation sequences to get things going permitting use as…

```shell
eval "$(prompter init)"
```

…to set things up.

The prompt system is designed to be both left-prompt and right-prompt and contains information
about both sides. Eventually prompter will fork into the background to stay running, but
for now it is invoked every time the prompt wishes to be displayed.
