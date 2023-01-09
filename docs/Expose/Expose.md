---
title: Exposé SIA-Arbeit
subtitle: ENTROPIE EINES WIRTSCHAFTSSYSTEMS
author: Daniel Meiborg
date: \today
bibliography: [../main.bib]
csl: ../ieee-with-url.csl
link-citations: true
documentclass: article
font_size: 12pt
---

\maketitle

>*You should call it entropy [...] no one really knows what entropy really is,
>so in a debate you will always have the advantage.*

John von Neumann to Claude Shannon, *Scientific American Vol. 225 No. 3, (1971)*

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
unterteilt, kann man dadurch mehrere Volkswirtschaftstypen und ihre
Eigenschaften vergleichen. Da es möglich ist, mit solchen Markov-Prozessen
Erhaltungssätze zu modellieren, werden Konstanten, die auf Größen wie Energie
oder verfügbaren Ressourcen basieren, eine zentrale Rolle bei der Modellierung
spielen.

Wichtig zu beachten ist, dass die genaue Übergangsmatrix des Markov-Prozesses
nicht direkt festgelegt wird, sondern erst durch die Simulation mit im Modell
bestimmten Regeln bestimmt wird.

# Motivation
Ziel dieses Modells ist es, tiefere Erkenntnisse über das Grenzwertverhalten von
Wirtschaften zu gewinnen, sowie diese nach Typen basierend auf ihrer Entropie zu
klassifizieren. So könnte diese Arbeit zum Beispiel zu dem Ergebnis führen, dass
ein kapitalistisches Wirtschaftssystem unter externen Einflüssen (zum Beispiel
durch unvorhersehbare Naturkatastrophen) eine deutlich höhere bzw. niedrigere
Entropiereduktion aufweist als eine sozialistische Volkswirtschaft. Dadurch
könnte man einen Maßstab entwickeln, der die Empfindlichkeit eines
Wirtschaftssystems dementsprechend beurteilt.

# Forschungsstand
Bisher wurden zwar schon Markov-Prozesse für die Modellierung von Wirtschaften
verwendet, allerdings wurde dabei nicht auf die Entropie im oben beschriebenen
Sinne geachtet [@barde2020macroeconomic;@Kostoska2020absorbingmc]. Genauso wurde
auch das Entropieverhalten von Markov-Prozessen analysiert, aber nicht auf die
Wirtschaft bezogen [@Rahman2022mccharacteristics].

# Zeitplan
- **Recherche** *4 Wochen* - Einlesen in das Themengebiet
- **Planung** *3 Wochen* - Konzeptionierung des Modells
- **Programmierung** *6 Wochen* - Programmierung der Simulation bzw. des
  Analyse-Frameworks für die Markov-Prozess-Analyse
- **Modellierung** *1 Woche* - Genaue Konfiguration/Eingabe der Parameter des
  Modells
- **Simulation** *1 Woche* - Simulation, Bestimmung der Übergangsmatrix und
  Untersuchung des Modells i.e. Spectral Analysis, Finden der stabilen
  Konfiguration mithilfe des Frameworks
- **Manipulation** *3 Wochen* - Eingreifen in die Simulation und Analyse der
  Entropie
- **Interpretation** *1 Woche* - Zurückführen der Ergebnisse auf die Wirtschaft
- **Schriftliche Arbeit** *6 Wochen* - Ausformulieren der schriftlichen Arbeit

# Methodisches Vorgehen
## Technologien
Für die Programmierung wird geplant die Sprache Python verwendet, sowie
branchenübliche Tools wie z.B. Git, Jupyter oder Docker.

## Quellen
Für die Literaturrecherche werden ausschließlich frei verfügbare Quellen i.e.
öffentlich zugängliche Publikationen und Dokumentationen verwendet.

## Ressourcen
Durch die Natur der Fragestellung wird zur Datenerhebung Rechenleistung
benötigt. Diese steht bereits in einem ausreichendem Maße bereits zur Verfügung.
Es entstehen also keine Kosten.

# Mögliche Probleme
**Modellierung** Wirtschaftssysteme erfüllen die Markov-Eigenschaft nicht i.e.
lassen sich nicht so Weise modellieren.

**Komplexität** Die benötigte Komplexitätsreduktion macht die Resultate
unbrauchbar.

**Speichereskalation** Durch zu viele Parameter wächst der Speicherbedarf
unkontrolliert.

# Quellen