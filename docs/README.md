<!-- PROJECT LOGO -->
<br />
<p align="center">

  <h1 align="center">3D World Generator + Smart Compass</h3>

  <p align="center">
    Tools created by Group: Rustbeef 🦀🐄
    <br />
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#about-the-project">About the project</a></li>
    <li>
      <a href="#world-generator-⛰️">World Generator</a>
      <ul>
        <li><a href="#graphical-example">Graphical example</a></li>
      </ul>
    </li>
    <li>
      <a href="#smart-compass-🧭">Smart Compass</a>
      <ul>
        <li><a href="#going-to-a-known-destination">To known destination</a></li>
        <li><a href="#going-to-an-unknown-destination">To unknown destination</a></li>
      </ul>
    </li>
    <li><a href="#contacts">Contacts</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Our team proposes a 3D world generator and a smart compass. These two tools can be used in combination to create the ultimate 3D gameplay experience for your project.</br>
To learn more about it, refer to the paragraphs below.


<!-- WORLD GENERATOR -->
## World Generator ⛰️

The main focus of our world generator is the elevation parameter.</br>
Our goal was to create an environment where the robot could move through hills and mountains of varying height and width. To do so, we derived the elevation values for the cells that compose the mountains from <b>three-dimensional gaussian curves</b>, that we parametrized differently in every iteration in order to obtain different shapes, sizes and positions for each hill and mountain.


### Graphical example

We know a picture speaks more than a thousand words, therefore we developed a graphical example to show a possible implementation of our 3D world generator.</br>

<p align="center">
<img src="img/demo.gif" width="500">
</p>


<!-- COMPASS -->
## Smart Compass 🧭

Our second tool consits in a smart compass that is able to take your robot from its current position to another tile, while also considering the best path in terms of costs of the tiles and altitude difference between them, in order to spend the least amount of energy. </br>
It is important to note that the directions it provides are just a suggestion, not something that you have to forcefully follow. Your robot still remains free to move in whatever way he decides to.

There are three ways of specifying the destination:
<ul>
  <li>By exact coordinates --> <i>"Take me to cell (12, 35)"</i></li>
  <li>By TileType --> <i>"Take me to the closest unvisited Snow cell"</i></li>
  <li>By Content --> <i>"Take me to the nearest Shop"</i></li>
</ul>
Additionally, for the "TileType" and "Content" method you can specify if to go to already visited cells or if to explore the map and go to unvisited places.</br>
More on that in the paragraphs below.


### Going to a known destination

The first case to consider is when the robot wants to go to a cell it has already visited before. This is pretty easy, since the coordinates of that cell are already available in the ```robot_map()``` method. </br>
Therefore, the starting coordinates and the destination coordinates are given in input into a <b>dijkstra algorithm</b> that has been customized for this project, taking into account the tile costs and the altitude conditions between the cells. </br>
The result is a series of directions that will take you to your destination in the cheapest way possible.</br>
Of course, the more tiles explored by the robot the better, because there are more possible "path options" to choose from.


### Going to an unknown destination

The second case arises when the robot decides to explore the map in order to find new TileTypes or Contents.</br>
In these cases, we don't have an exact coordinate for the destination, so applying the dijkstra algorithm described previously results impossible. Therefore, the compass will suggest a possible move for each tick based on a probabilistic model that still takes into consideration the cost of the tiles and the elevation difference. The robot will then follow a path based on a certain "luck", trying to find the destination is parts of the map that it hasn't explored before.

In the demo proposed in the <a href="#graphical-example">Graphical example</a> paragraph, you can actually see this version of the compass in action!</br> <i>(The cells are all unlocked for demonstration purposes, in reality the robot will only have access to already explored tiles)</i>


<!-- CONTACT -->
## Contacts

If you're interested in our tools, please reach out to us here!

<p align="center">
<img src="img/telegram.png" width="200">
</p>

### Group components

- [Thomas Pasquali](mailto:thomas.pasquali@studenti.unitn.it)
- [Salvatore Andaloro](mailto:salvatore.andaloro@studenti.unitn.it)
- [Claudio Foroncelli](mailto:claudio.foroncelli@studenti.unitn.it)
- [Florian Kandra](mailto:florian.kandra@studenti.unitn.it)