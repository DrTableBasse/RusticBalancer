# Cashless-services
<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/DrTableBasse/RusticBalancer">
    <img src="Images/logo_nfc.png" alt="Logo" width="150" height="150">
  </a>

  <h3 align="center">LoadBalancer en Rust</h3>

  <p align="center">
Système simple de présentation d'un loadBalancer en Rust.
    <br />
   </p>
</div>

## À propos du projet 

Ce projet est un projet en groupe de 2 de cours, il à été réalisé en quelques heures.

Il s'agit de créer un loadBalanceur en langage de programmation Rust. 
Il y a 2 serveurs et un servir de balancement.

## Prérequis

Pour utiliser ce système, vous aurez besoin de :

* Cargo d'installer 
* De la bienveillance

## Utilisation
Exécuter dans 3 terminals différents ces commandes : 
```sh 
cargo run echo_server
cargo run echo_server2
cargo run load_balancer
```

En faisant un ping sur le loadBalancer, il redirigera automatiquement sur le serveur 1 ou le serveur 2. 
Si plusieurs requête viennent du même point d'entrée dans les 2 secondes les requêtes sont envoyés au même serveur.

## Fonctionnalités principales

- LoadBalancing entre deux serveurs.

## Contribution 
Les contributions sont les bienvenues ! Pour contribuer, suivez les étapes suivantes :

1. Forkez le dépôt.
2. Créez une nouvelle branche pour votre fonctionnalité ou correctif.
3. Faites vos modifications et committez-les.
4. Poussez votre branche et créez une pull request.

## Auteurs

- Lefranc Nicolas, [@nico-vrn](https://github.com/nico-vrn)
- Nuez Samuel [@DrTableBasse](https://github.com/DrTableBasse)

## Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/DrTableBasse/RusticBalancer?style=for-the-badge
[contributors-url]: https://github.com/DrTableBasse/RusticBalancer/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/DrTableBasse/RusticBalancer.svg?style=for-the-badge
[forks-url]: https://github.com/DrTableBasse/RusticBalancer/network/members
[stars-shield]: https://img.shields.io/github/stars/DrTableBasse/RusticBalancer.svg?style=for-the-badge
[stars-url]: https://github.com/DrTableBasse/RusticBalancer/stargazers
[issues-shield]: https://img.shields.io/github/issues/DrTableBasse/RusticBalancer.svg?style=for-the-badge
[issues-url]: https://github.com/DrTableBasse/RusticBalancer/issues