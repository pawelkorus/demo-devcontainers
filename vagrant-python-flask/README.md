# vagrant image python

In this case you can connect to the running image via `Remote SSH` vscode extension. First you need to take ssh config via `vagrant ssh-config`. Output of this command you need to put into `~/.ssh/config`. Then you can use `Remote SSH - connect to host` option to connect to running vm. See [Medium page](https://medium.com/@lizrice/ssh-to-vagrant-from-vscode-5b2c5996bc0e) for reference.

## vagrant notes


### GUI mode

To enable GUI mode add this to virtualbox provide code

```
vb.gui = true       # Enable GUI mode
```