#import "template.typ": *
#show: ieee.with(
  title: "The applicability of the second law of thermodynamics to economic systems: Development and comparison of modeling techniques.",
  authors: (
    (
      name: "Daniel Meiborg",
      organization: [Schule Birklehof e.V.],
      location: [Hinterzarten, Germany],
      email: "mail@danielmeiborg.com"
    ),
  ),
  bibliography-file: "refs.yml",
  abstract: [
    This research paper investigates the applicability of the second law of thermodynamics to economic systems. Based on the Shannon entropy and fundamental concepts of Markov processes, various approaches to modeling economic structures are presented and analyzed. These include agent-based and diffusion-based Markov processes, entropy-based economic systems, evolutionary algorithms, and atomic operations. The advantages and disadvantages of each approach are discussed. The paper demonstrates that it is possible to apply the second law of thermodynamics to economic systems by creating appropriate models and introducing compensating terms to satisfy both reality and the second law of thermodynamics. Thus the modeling methods presented can help to better understand and analyze the underlying mechanisms and relationships in economic systems.
  ],
)

= Introduction

_You should call it entropy [...] no one really knows what entropy really is,
so in a debate you will always have the advantage._

John von Neumann to Claude Shannon @tribus1971information[p.~180]
#line(length: 100%)

The validity of the second law of thermodynamics for physical processes is generally accepted and has far-reaching implications for our understanding of the world. This fundamental principle states that in a closed system, entropy always increases. While the second law of thermodynamics can be generally proved for a group of stochastic processes, there are as yet no comprehensive approaches to apply this concept quantitatively to economic systems. Therefore, it is of great importance to investigate more precisely the applicability of the second law of thermodynamics as one of the most fundamental principles of physics to economic systems.

To study complex phenomena, science often makes use of mathematical models. In economics, there are numerous models that are used to represent economic relationships. In this paper, the mathematical concepts necessary to abstractly formulate the second law of thermodynamics are first introduced. Based on this, several models are formulated with the purpose of satisfying the second law of thermodynamics. Analysis of these approaches includes discussion of their advantages and disadvantages and identification of areas in which they can potentially be extended or improved.

The modeling methods presented in this thesis provide the basis for further research in this area. The application of the models is left for future development.

The models presented can help to better understand and analyze the underlying mechanisms and relationships in economic systems. In this context, the application of the second law of thermodynamics to economic structures can provide new perspectives and insights that are important for both theoretical research and practical applications.

= Theoretical Foundations

In order to be able to set up models of the second law of thermodynamics, it is absolutely necessary to be able to describe it mathematically. It is necessary to use an abstract definition of processes and entropy in order to create the basis for an extension of its field of application. In this chapter, a brief overview of the concepts of information, entropy, Markov processes and, based on them, the formal definition of the second law of thermodynamics will be given.

== Shannon Information

In computer science, information content of a message, also known as Shannon information, is defined as the measure of "surprise" associated with an event. Let $p(x)$ denote the probability of event $x$. The information content is formally expressed as $I(x) = -log_2(p(x))$ (in bits). It is important to note that this quantity is distinct from the colloquial notion of content. The information content does not convey the utility of the message. Instead, this measure can be understood as the natural limit for compressing a message or as the minimum number of yes-no questions required to determine the outcome (for non-integer values, the average count).

An example illustrates this concept: A typical fair coin has a probability of $1/2$ for heads and $1/2$ for tails. The information content for both events is thus $-log_2(1/2) = 1$ bit each. For an unfair coin with a probability of $1/3$ for heads and $2/3$ for tails, the information content for heads is approximately $-log_2(1/3) approx 1.585$ bits, and for tails, it is approximately $-log_2(2/3) approx 0.585$ bits.

In this work, the information content is primarily employed for defining entropy, except for the model in @markup-language.

=== Properties of the Shannon Information

The information content is a strictly monotonically decreasing function of probability, defined for the interval $(0, 1]$. The information content of an event that is completely certain, i.e., has a probability of $1$, is $0$. The information content of an event that cannot occur would be $oo$.

The information content of two independent events is the sum of their respective information contents.

== Entropy

Entropy is a central concept of the second law of thermodynamics and is found in numerous individual disciplines. In physics, it manifests in the form of the quantum-mechanically defined Von Neumann entropy, in the temperature differences among various systems, and in statistical mechanics. In chemistry, there exists reaction entropy. Each field employs a somewhat distinct definition. In computer science, Shannon entropy, introduced by Claude Shannon, is predominantly used. For the purposes of this work, only Shannon entropy is employed and is synonymous with entropy.

=== Formal Definition

Entropy is the expected value of the information content of a random variable. It is defined as $H(X) = sum_(x in X) p(x) dot I(x) = -sum_(x in X) p(x) dot log_2(p(x))$ (in bits), where $p(x)$ represents the probability function of the random variable @gray1990informationtheory[~p.18].

=== Illustrative Explanation

Entropy is often described as a measure of the disorder within a system. However, Shannon entropy is somewhat more abstract; it characterizes the distribution of probabilities.

Entropy is closely linked to statistical mechanics. In this context, a gas that is uniformly distributed exhibits high entropy, whereas a state where all gas molecules gather on one side of the room has low entropy. This distinction arises because there are significantly more possible states, or a higher probability, for the first scenario. However, this ratio aligns more with the information content of states rather than the entropy of the system.

Let us revisit the coin example: For a fair coin, the entropy is $(1 + 1)/2 = 1$ bit. For the unfair coin, the entropy is approximately $1.585 dot 1/3 + 0.585 dot 2/3 approx 0.918$ bits. Thus, the entropy is lower than that of a uniform distribution. This observation holds for all probabilities; the uniform distribution always possesses the highest entropy. Distributions in which only a single state is possible exhibit the lowest possible entropy of $0$. Therefore, entropy can be seen as capturing, to some extent, the disorder of probabilities.

== Markov Processes

In @second-law-of-thermodynamics, it is demonstrated that the second law of thermodynamics can be generally formulated for Markov processes. Consequently, nearly all models within this work are based, in one way or another, on Markov processes.

=== Definition <markov-process-definition>

A Markov process, also known as a Markov chain, is a stochastic process that exhibits the Markov property. It consists of a set of states and a probability matrix that describes the transition probabilities between these states. The Markov property stipulates that transition probabilities solely depend on the current state.

Let A be the transition matrix with $A_(i j)$ denoting the probability of transitioning from state $j$ to state $i$, and $arrow(P)(t)$ representing the state probabilities at time $t$ in vector form. Then the so-called master equation is expressed as $arrow(P)(t + 1) = A arrow(P)(t)$ @tolver2016markovchains[~p.15].

#figure(
  image("markov-chain.png"),
  caption: [A simple Markov process],
)

For this work, only finite discrete Markov processes are to be considered. This entails processes where there are only a finite number of states, and time advances in discrete steps.

=== Stationary Distribution

The stationary probability distribution of a Markov process is the distribution $arrow(P)$ that satisfies $arrow(P) = A arrow(P)$. The stationary distribution represents an equilibrium state of the process.

== Second Law of Thermodynamics <second-law-of-thermodynamics>


The second law of thermodynamics lacks a unique formulation. However, all formulations encompass the following core statement:

_Entropy cannot decrease in a closed thermodynamic system._

To some extent, this statement can be generalized to stochastic processes. The proofs for this were conducted in this paper @cover1994processes[p.~98-107], from which the sketch of the proof for the second law of thermodynamics is derived below.

#theorem[For all finite discrete Markov processes, if the transition probability matrix is doubly stochastic (i.e., when the stationary distribution is the uniform distribution), then $H(\mu_n)$ monotonically increases.]

#proof[Let $m$ be the number of possible states, $mu_n$ a probability distribution at time $n$, and $mu$ the stationary distribution, in this case, the uniform distribution. Here, $D(X || Y)$ represents the Kullback-Leibler divergence (a measure of dissimilarity between two probability distributions). Then,

$D(mu_n || mu)
&= sum_x mu_n(x) dot log_2((mu_n(x))/(1/m))
&= -H(mu_n) + log_2(m)$

Since $D(mu_n || mu)$ monotonically decreases, $H(mu_n)$ monotonically increases @cover1994processes[p.~103].
]

This theorem can be understood as follows: Consider a Markov process where there is no information about the prevailing state (although the potentially attainable states and transition probabilities are known), and the process is executed for a certain time. If, afterwards, one cannot determine with greater accuracy the likelihood of the current state, then the second law of thermodynamics holds true.

== Summary

This chapter has presented the general concept of entropy and the second law of thermodynamics. Information content is defined as the measure of "surprise" associated with an event. Entropy describes the expected value of the information content of a random variable and serves as a measure of the disorder of probabilities.

Markov processes were introduced as stochastic processes with the Markov property, wherein the transition probabilities to the next state depend solely on the current state. The stationary distribution of such a process describes the equilibrium state of the probability distribution.

The second law of thermodynamics states that entropy in a closed system cannot decrease. It has been demonstrated that this principle can also be applied to finite discrete Markov processes when the stationary distribution is the uniform distribution, or when the transition probability matrix is doubly stochastic. In such cases, entropy increases monotonically, implying that over time, one cannot acquire more information about a system than what was known at time $0$.

However, one should not overlook the limitations of the Markov property in the @markov-process-definition. The essential point of the Markov property is that the probabilities for the next states depend solely on the current state. This also implies that agents learning from the past can significantly complicate the system. Nevertheless, many systems can be simplified to satisfy the Markov property.

Thus, a Markov process must have a doubly stochastic transition probability matrix or equivalently the uniform distribution as its stationary probability distribution to fulfill the second law of thermodynamics. To achieve this, the concept of compensating terms is introduced.

= Compensating Terms <compensating-terms>

As the second law of thermodynamics applies only to Markov processes with a stationary distribution that is the uniform distribution, the models must inherently satisfy this criterion. The objective is to construct a model for the underlying phenomenon in such a way that this Markov process adheres to the second law of thermodynamics. However, since this is a specific case of Markov processes, it is unlikely to be fulfilled in the initial attempt. For this reason, compensating terms can be introduced. These terms serve to adjust the model in a manner that aligns both with reality and the second law of thermodynamics. These often take the form of systems possessing low entropy, capable of absorbing entropy from other systems. For physical processes, sources of low entropy, for instance, significant temperature differentials as found in an engine, are utilized.

= Agent-Based Markov Process

In an agent-based Markov process, the aim is to model economic agents (such as states, banks, etc.). Agents are understood here in the context of game theory as entities capable of interacting with their environment. The initial step involves translating the behavior of individual agents, for instance, using decision diagrams, into a format that can be further processed.

Subsequently, this model has to be transformed into a Markov process.

This Markov process can then be comparatively straightforwardly simulated. From the entropy to be calculated from it (see @entropy-agents) or directly from the transition probability matrix, one can assess whether the second law of thermodynamics holds. Should this not be the case, the original model must accordingly be progressively adjusted.


== Entropy <entropy-agents>

Entropy is defined here based on the Shannon entropy of potential states. The probability distribution $X_0$ is initially known, and entropy is represented as a function over time. This principle is also followed by all other models presented here that are based on Markov processes.

== Advantages

As an actual economic system also consists of a multitude of agents, this system has the potential, when ideally implemented, to yield promising outcomes.

== Issues

However, several challenges arise in this context: First, the modeling of human agents is extremely labor-intensive, difficult, or outright impossible. Even in the case of very simple systems, a multitude of parameters must be arbitrarily set due to the typically limited availability of data and processing capacity. Furthermore, the compensating terms are more intricate to implement compared to the later approaches, as they require the introduction of new agents or substantial alterations in the decisions of existing agents, further compromising the model's interpretive significance.

= Diffusion-Based Markov Process

The approach in diffusion-based Markov processes is similar to that of agent-based Markov processes. However, the main distinction lies in the fact that instead of modeling all agents individually to a large extent, only the flows of capital or resources are specified at a coarse level. The term 'diffusion' is to be understood here as services or products generally requiring a form of reciprocation, causing capital or resources to diffuse among different agents.

== Example

This diagram illustrates schematic relationships between economic entities. In the next step, probabilities for diffusion and quantities need to be provided.

#figure(
  image("diffusion-example.png"),
  caption: [Example of a model for a diffusion-based Markov process]
)

== Advantages

A significant advantage of this approach is that decisions made by agents are assumed to be inherently unpredictable from the outset. Instead, only the broader contexts are considered. In fact, this unpredictability is even beneficial here, as randomness is an essential element for Markov processes.

== Issues

However, in attempting to model the economic system at a lower level, this concept can evolve into a more complex version of the agent-based approach. Thus, a balance must always be struck between these two approaches.

= Entropy-Based Economic System <entropy-based-economic-system>

The next approach involves considering only those parts of the economy that deal with probabilities themselves. This broad field encompasses everything from insurance over gambling to cryptocurrencies. Both real systems and hypothetical economic systems, where pure information serves as currency, can be examined.

== Example

An example of this is a virtual horse race: In this scenario, the only information that leaves the otherwise closed system (e.g., a computer) is the (unique) winner. With 8 horses, this represents 3 bits of information. Prior to the commencement of the random experiment, there is 3 bits of uncertainty; afterward, there are 0 bits (pertaining to the horse race). Now, a Markov process can be created from the horse race and its environment. To model this according to the second law (which, to our knowledge, applies to physics), this horse race simulation must increase entropy by at least 3 bits. According to the Landauer principle @landauer1961limit[p.~188], a minimum amount of energy is required to erase a single bit of information. This minimum is calculated using Landauer's formula as $E = k_B T ln(2)$, where $k_B$ corresponds to the Boltzmann constant. At room temperature, one bit corresponds to at least $E = k_B dot 295K dot ln(2) approx 2.823 dot 10^{-21}J$, and the 3 bits amount to approximately $2.823 dot 10^{-21}J dot 3 dot 8.469 dot 10^{-21}J$, which must be converted into heat. With an electricity price of about $0.33 (€)/("kWh")$ @destatis2022electricity in Germany in 2022, this is approximately $8.469 dot 10^(-21)J dot 0.33 dot 3.6 dot 10^(-6) €/J approx 10^(-26)€$, which is the minimum value of the outcome. Information can be converted back into useful work using Maxwell's demon @bennet1987demon (specifically, the overwriting of memory). In a society, information could, in principle, be used as an actual valuable entity.

== Advantages

Since the industry is directly oriented toward probabilities, the concept of entropy holds greater significance here than in generic agent- or diffusion-based models.

== Issues

However, it becomes evident for real economic sectors that the entropy obtained through speculation generally does not correspond to what such a calculation would yield, as these systems are not entirely closed and information can leak outward.

= Markup Language <markup-language>


While the previous modeling approaches are based on modeling a specific economic system and calculating its entropy, this approach calculates the entropy of all possible economic systems in a certain sense.

In the first step, a markup language is defined. This could, for instance, be an XML dialect. This syntax is used to describe a particular system.

It is important to note that initially only the information content, not the entropy of this document, is calculated. To do this, a certain length is chosen for the documents or their number is otherwise limited. Subsequently, a probability is assigned to each of these documents. This probability can be the same for each document or more complex approaches can be used, as illustrated in @evolutionary-algorithms. As explained in the theoretical foundations, this probability can then be transformed into the information content. Finally, the entropy can be calculated for the entire set of documents.

== Example

Below is a possible representation of a system consisting of three agents in a fictitious XML dialect.

```xml
<economy>
  <agents>
    <agent id="A1" type="producer" />
    <agent id="A2" type="consumer" />
    <agent id="A3" type="producer" />
  </agents>
  <products>
    <product id="P1" producer="A1" />
    <product id="P2" producer="A3" />
  </products>
  <connections>
    <trade producer="A1" consumer="A2" product="P1" price="10" />
    <trade producer="A3" consumer="A2" product="P2" price="5" />
  </connections>
</economy>
```

== Advantages

The main advantage here is that one can calculate the entropy of the structure of economic systems, rather than the entropy of their behavior. This avoids the challenging step of simulating individual agents.

== Issues

However, without further modifications, the second law of thermodynamics cannot be applied, as the documents are not Markov processes.

= Evolutionary Algorithms <evolutionary-algorithms>

To establish a connection between the markup language model and time, one approach is to embed it in the evolution of economic systems. Such a model essentially requires the following elements: a data format (here, the markup language) for the candidates (an individual document), a function that indicates a candidate's fitness (which could be, for instance, the purchasing power parity of the gross domestic product per capita or the Gini coefficient for wealth distribution), and an algorithm for mutation.

During the execution of the simulation, an initial random set of candidates is generated. An iteration involves applying a form of selection, such as choosing the top $x%$ determined by the fitness function. Subsequently, these candidates undergo mutation. This mutation process could involve randomness or employ more complex methods, where agents adjust the economic system.

Entropy can be computed by determining the candidates' share of the population based on their fitness and subsequently deriving the probability of selecting them through random sampling. The entropy can then be calculated from this probability distribution.

== Advantages

This approach enables the consideration of markup languages as models evolving over time. This facilitates comparisons with other models that often incorporate time as a parameter.

== Issues

However, these simulations also do not represent a Markov process. This is because, at most, one could regard the distribution of the entire population as a state, making the Markov process entirely deterministic and thus uninteresting. It is not possible to view a single candidate as a state, because the probability to go to another state depends on the probabilities for other states. Thus the Markov property and in extension the second law of thermodynamics does not hold true.

Furthermore, an evolutionary algorithm tends to select one or a few candidates, which then constitute the entire population. This contrasts with the initial distribution, where there are many different candidates, resulting in a decrease in entropy. However, this is not always the case, as mutation can temporarily widen the distribution.

= Atomic Operations

In contrast to previous models, in the context of atomic operations processes are examined at a granular level. Instead of analyzing the structure of an economic system, this approach involves modeling basic economic processes using Markov processes. In this context, 'atomic' is not to be interpreted in the context of physics, but rather as a _indivisible_ action.

== Example

One such atomic operation could be the production of goods from raw materials. The following diagram illustrates such a process. In this Markov process, there are two groups of states: _Raw Material_ and _Product_. At time $t=0$, it is unknown which of the three possible states R1, R2, or R3 the raw material will actually assume, i.e., each of them has a probability of $1/3$, resulting in an entropy of $H(X_0) = log_2(3)$. At time $t=1$, these states transition randomly to either P1 or P2. Thus, the entropy becomes $H(X_0) = log_2(2) = 1 "bit"$.

#figure(
  image("atomic-operations-basic.png"),
  caption: [Simple example of a production process]
)

To satisfy the second law of thermodynamics, the model can be expanded in various ways. One such approach is the introduction of waste. It is assumed that during production, a certain amount of undefined waste is generated. "Undefined" in this context means that during production, there is no control over producing a specific type of waste; rather, the waste can be anything. Thus, the waste possesses a high number of potential states (arbitrarily chosen as 5). Consequently, the entropy ratios are as follows: $H(X_0) = log_2(3) approx 1.585$ bits and $H(X_1) = log_2(2 dot 5) = log_2(2) + log_2(5) approx 3.322$ bits.

#figure(
  image("atomic-operations-waste.png"),
  caption: [The same production process, using waste]
)


== Advantages

In contrast to the other models, this approach is comparatively intuitive and comprehensible. Furthermore, it does not require predicting agent behavior, which significantly enhances its feasibility.

== Challenges

However, the complexity of this approach grows immensely when considering a larger system. Here, the boundaries between the first two models become blurred.

= Summary

Agent-based and diffusion-based models provide suitable approaches for analyzing the entropy of behavior within an individual economic system and for examining economic processes at a higher level. Nonetheless, due to their complexity, interpreting and scaling these models can be challenging. For comparing the structure of an economic system, a markup language is more suitable, as it obviates the need to simulate agents. Ultimately, atomic operations enable the examination of economic processes at a lower level.

= Limitations

While the issues of individual modeling approaches have been introduced in their respective chapters, it is nevertheless important to discuss the general limitations of applying the second law of thermodynamics to economic systems.

A fundamental problem arises from the fact that the models are often built on unrealistic assumptions. This pertains particularly to the compensatory terms used to align the models with the second law of thermodynamics. These compensatory terms can appear arbitrary and thus represent a potential weakness in the modeling process. This is especially relevant in the context of artificially introducing negative entropy sources.

The scalability and complexity of the models also present limitations. Applying the second law of thermodynamics to economic systems necessitates the development of intricate models and simulations that account for both the underlying processes and the second law itself. This complexity can render the models difficult to interpret. Particularly when employing agent-based and diffusion-based models, the resulting simulations can prove challenging to analyze.

The constraints imposed by limited resources further complicate matters. Crafting and simulating the models demand substantial time and a grasp of the underlying processes and principles. Moreover, the computational power and technical resources required for conducting simulations and analyzing outcomes are finite. These constrained resources might hinder an in-depth investigation and analysis of the models and simulations.

Lastly, it's worth noting that most economic theories posit that economic systems strive for equilibrium. This implies a substantial reduction in entropy within an economic system. This can complicate the introduction of compensatory terms, as one might not be able to directly employ an economic factor and may need to resort to factors such as environmental impacts.

= Outlook

The approaches presented in this work for applying the second law of thermodynamics to economic systems offer promising potential for future research and practical applications. The following discusses some perspectives and further research avenues that concern both the implementation of these approaches in practice and the modeling of risk management, as well as the utilization of artificial intelligence for model development.

== Implementation of Approaches

After the development of modeling approaches, the actual implementation of the modeling techniques is the logical next step. To examine the effectiveness of these models in real economic systems, it is necessary to validate and calibrate them using empirical data. Historical data from various economic sectors and countries can be utilized, for instance, to test and optimize the models. The effectiveness of implementation depends, among other factors, on the quantity and significance of available data.

== Artificial Intelligence

The application of artificial intelligence (AI), particularly deep learning and evolutionary algorithms (distinct from the model "Evolutionary Algorithms" in @evolutionary-algorithms), can be valuable for creating models based on the described approaches. These technologies could be employed to make the adjustment of models through balancing terms as realistic as possible.

The integration of AI into the presented approaches for modeling economic systems with consideration of entropy could thus enhance the efficiency and applicability of these models in practice and simultaneously better manage model complexity.

== Modeling Risk Management

Applying the second law of thermodynamics to economic processes also opens new possibilities for modeling risk management in businesses and financial markets. Given the essential role of information, it is pertinent to further investigate its connection with the second law of thermodynamics.

Future research efforts could focus on adapting and advancing the presented models for analyzing risks in different market segments and industries. This could involve studying the volatility of stock prices, exchange rate risks, or credit risks in relation to entropy. These risks could then be linked to entropy and information (akin to entropy-based economic systems in @entropy-based-economic-system).

= Conclusion

This work examined the applicability of the second law of thermodynamics to economic systems. Various modeling approaches were introduced that allow for the transfer of the second law of thermodynamics to economic systems, offering a new perspective on economic relationships. Agent-based and diffusion-based models, markup languages, entropy-based systems, evolutionary algorithms, and atomic operations were investigated and compared.

The analysis of these different approaches demonstrates the feasibility of applying the second law of thermodynamics to economic systems. The presented modeling methods provide a foundation for further research in this field and can contribute to better understanding and analyzing the underlying mechanisms and relationships in economic systems.
