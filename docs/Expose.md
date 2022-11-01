---
title: Exposé SIA Arbeit
subtitle: ENTROPIE EINER WIRTSCHAFT
author: Daniel Meiborg
bibliography: main.bib
csl: ieee-with-url.csl
link-citations: true
documentclass: article
header-includes: \usepackage[a4paper, top=0.5cm, left=1cm, right=1cm, bottom=0.5cm]{geometry}
---

\pagenumbering{gobble}

\maketitle

>*You should call it entropy [...] no one really knows what entropy really is,
>so in a debate you will always have the advantage.*

John von Neumann zu Claude Shannon, *Scientific American Vol. 225 No. 3, (1971)*

# Thema
*Lassen sich einfache ökonomische Prozesse mit einem Markov-Prozess mit einer
uniformen stationären Wahrscheinlichkeitsverteilung modellieren und aus der
durch äußere Einflüsse entstehenden Entropiereduktion Rückschlüsse auf unsere
Wirtschaft treffen?*


Grundbaustein dieser Herangehensweise ist der zweite Hauptsatz der
Thermodynamik. Dieser gilt unter anderem für Markov-Prozesse (auch *Markov
Chains* genannt) unter bestimmten Voraussetzungen[@cover1994processes]. Durch
manuelles Eingreifen lässt sich die Entropie des Systems allerdings reduzieren.
Diese Entropiereduktion ist äquivalent zu der Menge an Information, die man
durch das Eingreifen erhält. Wenn man das in mehrere Subumgebungen unterteilt,
kann man dadurch mehrere Wirtschaftstypen und ihre Eigenschaften vergleichen.

## Motivation
Ziel dieses Modells sind tiefere Erkenntnisse über das Grenzwertverhalten von
Wirtschaften, sowie diese nach Typen basierend auf ihrer Entropie zu
klassifizieren.

## Forschungsstand
Im bisherigen Forschungsstand wurden zwar schon Markov-Prozesse für die
Modellierung von Wirtschaften verwendet, allerdings wurde dabei nicht auf die
Entropie im oben beschriebenen Sinne
geachtet[@barde2020macroeconomic][@Kostoska2020absorbingmc]. Genauso wurde auch
das Entropieverhalten von Markov-Prozessen analysiert, aber nicht auf die
Wirtschaft bezogen [@Rahman2022mccharacteristics].

# Zeitplan
- **Recherche** Einlesen in das Themengebiet
- **Planung** Konzeptionierung des Modells und der Versuche
- **Framework** Programmierung des Frameworks für die Markov-Prozess-Analyse
- **Modellierung** Genaue Konfiguration/Eingabe der Parameter des Modells
- **Analyse** Untersuchung des Modells mit bisherigen Methoden
- **Manipulation** Eingreifen in die Simulation und Analyse der Entropie
- **Interpretation** Zurückführen der Ergebnisse auf die Wirtschaft
- **Wiederholung** Wiederholung mit anderen Modellen

# Mögliche Probleme
- **Modellierung** Die Markov-Eigenschaft ist nicht sinnvoll in diesem Modell
  erfüllbar.
- **Komplexität** Die benötigte Komplexitätsreduktion macht die Resultate
  unbrauchbar.
- **Speichereskalation** Durch zu viele Parameter wächst der Speicherdarf
  unkontrolliert.

# Quellen