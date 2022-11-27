# TP - WIK-DPS-TP01

## Pré-requis

- Rust(cargo)
- Docker
- Git

## Installation/Configuration

Cloner le projet.

### Image avec un seul stage

Taper la commande suivante une fois dans le projet pour build l'image.
```docker build -t image .```

Et ensuite lancer le projet avec 
```docker run -p8888:8888 image```

### Image multi-stage

Taper la commande suivante une fois dans le projet pour build l'image.
```docker build -t image2 -f Dockerfile2 .```

Et ensuite lancer le projet avec 
```docker run -p8888:8888 image2```


