---
title: Exposé SIA-Arbeit
subtitle: ENTROPIE EINES WIRTSCHAFTSSYSTEMS
author: Daniel Meiborg
date: \today
bibliography: main.bib
csl: ieee-with-url.csl
link-citations: true
documentclass: article
font_size: 12pt
# header-includes: \usepackage[a4paper, top=0.5cm, left=1cm, right=1cm, bottom=0.5cm]{geometry}
---

<!-- \pagenumbering{gobble} -->

\maketitle

>*You should call it entropy [...] no one really knows what entropy really is,
>so in a debate you will always have the advantage.*

John von Neumann zu Claude Shannon, *Scientific American Vol. 225 No. 3, (1971)*

<!-- TODO: genauer erläutern -->
# Thema
*Lassen sich einfache ökonomische Prozesse durch einem Markov-Prozess mit einer
uniformen stationären Wahrscheinlichkeitsverteilung modellieren und aus der
durch äußere Einflüsse entstehenden Entropiereduktion Rückschlüsse auf
verschiedene Wirtschaftssysteme treffen?*


Grundbaustein dieser Herangehensweise ist der zweite Hauptsatz der
Thermodynamik. Dieser gilt unter anderem für Markov-Prozesse (auch *Markov
Chains* genannt) unter bestimmten Voraussetzungen [@cover1994processes]. Durch
manuelles Eingreifen lässt sich die Entropie des Systems allerdings reduzieren.
Diese Entropiereduktion ist äquivalent zu der Menge an Information, die man
durch das Eingreifen erhält. Wenn man dieses System in mehrere Subumgebungen
unterteilt, kann man dadurch mehrere Wirtschaftstypen und ihre Eigenschaften
vergleichen.

## Motivation
Ziel dieses Modells ist es tiefere Erkenntnisse über das Grenzwertverhalten von
Wirtschaften zu gewinnen, sowie diese nach Typen basierend auf ihrer Entropie zu
klassifizieren.

## Forschungsstand
Bisher wurden zwar schon Markov-Prozesse für die Modellierung von Wirtschaften
verwendet, allerdings wurde dabei nicht auf die Entropie im oben beschriebenen
Sinne geachtet [@barde2020macroeconomic;@Kostoska2020absorbingmc]. Genauso wurde
auch das Entropieverhalten von Markov-Prozessen analysiert, aber nicht auf die
Wirtschaft bezogen [@Rahman2022mccharacteristics].

# Zeitplan und Vorgehen
- **Recherche** *1 Monat* - Einlesen in das Themengebiet
- **Planung** *1 Monat* - Konzeptionierung des Modells und der Versuche
- **Framework** *2 Monate* - Programmierung eines Simulation Frameworks für die
  Markov-Prozess-Analyse
- **Modellierung** *2 Wochen* - Genaue Konfiguration/Eingabe der Parameter des
  Modells
- **Analyse** *1 Woche* - Untersuchung des Modells i.e. Spectral Analysis,
  Finden der stabilen Konfiguration
- **Manipulation** *2 Wochen* - Eingreifen in die Simulation und Analyse der
  Entropie
- **Interpretation** *2 Wochen* - Zurückführen der Ergebnisse auf die Wirtschaft
- **Schriftliche Arbeit** *2 Monate* - Ausformulieren der schriftlichen Arbeit

<!-- TODO: Methodisches Vorgehen -->
# Ressourcen
Durch die abstrakt Natur der Fragestellung wird ausschließlich Rechenleistung
benötigt. Diese steht in einem ausreichendem Maße bereits zur Verfügung. Es
entstehen also keine Kosten.
<!-- TODO: Rudimentäre Gliederung -->
# Mögliche Probleme
- **Modellierung** Wirtschaftssysteme erfüllen die Markov-Eigenschaft nicht i.e.
  lassen sich nicht so Weise modellieren.
- **Komplexität** Die benötigte Komplexitätsreduktion macht die Resultate
  unbrauchbar.
- **Speichereskalation** Durch zu viele Parameter wächst der Speicherbedarf
  unkontrolliert.

# Quellen