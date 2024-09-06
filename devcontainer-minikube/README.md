# devcontainer-helm-minikube

Demo project that shows how to work with minikube & helm inside devcontainer.

This demo use [devcontainer template](https://github.com/devcontainers/templates/tree/main/src/kubernetes-helm-minikube) to setup all required things.

Devcontainer features might not work with all images as usually they require `apt` installed in guest container to install additional software.

## notes

minikube needs to be strated manually via `minikube start`