#set document(title: "Hacking: An Overview", author: "Lerpz")
#set page(numbering: "1")
#set heading(numbering: "1.")

#align(center)[
  #text(size: 24pt, weight: "bold")[Hacking: An Overview]
  #v(0.5em)
  #text(size: 12pt, fill: gray)[A reference document for RAG embedding tests]
]

#v(1em)
#outline()
#pagebreak()

= Introduction to Hacking

Hacking refers to the practice of exploring, modifying, or exploiting systems — particularly computer systems and networks — often beyond their originally intended use. The term has a complex history, originating in the 1950s and 1960s among technical communities at institutions like MIT, where it described creative and clever problem-solving. Over time, popular media reshaped the word to carry a more sinister connotation, though the security community maintains important distinctions between different types of hackers.

Today, hacking encompasses a broad spectrum of activities ranging from legitimate security research and penetration testing to criminal exploitation of vulnerabilities. Understanding these distinctions is critical for anyone working in or around information security.

= Types of Hackers

== White Hat Hackers

White hat hackers, also known as ethical hackers or penetration testers, use their skills to improve security. They are typically employed by organisations or hired as consultants to find and fix vulnerabilities before malicious actors can exploit them. White hats operate with explicit permission and follow a strict code of ethics. Certifications like the Certified Ethical Hacker (CEH) and Offensive Security Certified Professional (OSCP) are common credentials in this field.

== Black Hat Hackers

Black hat hackers operate without authorisation and with malicious intent. Their goals commonly include financial gain, data theft, espionage, or sabotage. Black hat activity is illegal in virtually all jurisdictions and can result in severe criminal penalties. Notable examples include large-scale data breaches, ransomware attacks on hospitals, and state-sponsored intrusions into critical infrastructure.

== Grey Hat Hackers

Grey hat hackers occupy the space between white and black hat activity. They may probe systems without permission but typically do not cause deliberate harm or steal data. Grey hats often disclose vulnerabilities to the affected organisation after discovering them, sometimes requesting a fee. This unsolicited activity is still technically illegal in most countries, even when well-intentioned.

== Script Kiddies

Script kiddies are unskilled individuals who use pre-written tools and exploit kits without understanding the underlying techniques. While they lack sophistication, they can still cause significant damage because automated tools lower the barrier to entry for attacks like distributed denial-of-service (DDoS) campaigns.

== Hacktivists

Hacktivists use hacking as a form of political or social protest. Groups like Anonymous have conducted high-profile operations against governments, corporations, and organisations they perceive as acting unethically. Tactics include website defacement, DDoS attacks, and leaking sensitive documents.

= Common Hacking Techniques

== Phishing

Phishing is one of the most prevalent attack vectors. Attackers craft deceptive emails, messages, or websites that impersonate trusted entities to trick victims into revealing credentials, clicking malicious links, or downloading malware. Spear phishing is a targeted variant directed at specific individuals or organisations, often leveraging personal information to increase credibility.

== SQL Injection

SQL injection (SQLi) exploits vulnerabilities in web applications that construct database queries using unsanitised user input. By inserting malicious SQL syntax into input fields, an attacker can manipulate backend databases to dump sensitive data, bypass authentication, or in some cases execute commands on the underlying server. SQLi has been responsible for some of the largest data breaches in history.

== Cross-Site Scripting (XSS)

Cross-site scripting attacks inject malicious scripts into web pages viewed by other users. These scripts run in the victim's browser and can steal session cookies, redirect users to phishing pages, or perform actions on the victim's behalf. XSS vulnerabilities are classified as reflected, stored, or DOM-based depending on how the malicious payload is delivered and executed.

== Man-in-the-Middle Attacks

In a man-in-the-middle (MitM) attack, the attacker secretly intercepts and potentially alters communication between two parties who believe they are communicating directly. Techniques include ARP spoofing on local networks, SSL stripping to downgrade encrypted connections, and rogue Wi-Fi access points in public locations. MitM attacks can be used for credential harvesting, session hijacking, or injecting malicious content.

== Buffer Overflow

Buffer overflow vulnerabilities occur when a program writes more data to a buffer than it can hold, overwriting adjacent memory. Attackers exploit this to overwrite return addresses and redirect execution to shellcode. Although modern operating systems implement mitigations like Address Space Layout Randomisation (ASLR) and Data Execution Prevention (DEP), buffer overflows remain relevant, particularly in legacy software and embedded systems.

== Social Engineering

Social engineering manipulates human psychology rather than technical vulnerabilities. Attackers impersonate authority figures, IT staff, or colleagues to convince targets to divulge information, reset passwords, or grant access. Pretexting, baiting, and tailgating are all forms of social engineering. Security awareness training is the primary defence against these attacks.

== Ransomware

Ransomware is malware that encrypts a victim's files and demands payment — typically in cryptocurrency — in exchange for the decryption key. Modern ransomware operations, such as those associated with groups like LockBit and REvil, often use a double-extortion model: encrypting data while also threatening to publish stolen information publicly if the ransom is not paid.

= The Hacking Process

Professional penetration testers and malicious hackers alike generally follow a structured methodology:

+ *Reconnaissance* — Gathering information about the target through passive (OSINT) and active means. Tools include Shodan, Maltego, and DNS enumeration utilities.
+ *Scanning and Enumeration* — Actively probing the target to identify open ports, running services, and software versions. Nmap is the canonical tool for this phase.
+ *Vulnerability Analysis* — Identifying weaknesses in discovered services using tools like Nessus, OpenVAS, or manual analysis.
+ *Exploitation* — Leveraging vulnerabilities to gain unauthorised access. The Metasploit Framework is widely used in this phase.
+ *Post-Exploitation* — Maintaining access, escalating privileges, moving laterally through the network, and exfiltrating data.
+ *Reporting* — In legitimate engagements, documenting findings, evidence, and remediation recommendations for the client.

= Defences and Countermeasures

Effective security requires a layered defence strategy:

- *Patch Management*: Keeping software and firmware up to date closes known vulnerabilities.
- *Principle of Least Privilege*: Users and processes should have only the permissions necessary for their function.
- *Multi-Factor Authentication (MFA)*: Adds a second layer of verification, significantly reducing the impact of compromised credentials.
- *Network Segmentation*: Isolating network segments limits lateral movement in the event of a breach.
- *Intrusion Detection and Prevention Systems (IDS/IPS)*: Monitor traffic for known attack signatures and anomalies.
- *Security Awareness Training*: Educates employees to recognise phishing and social engineering attempts.
- *Regular Penetration Testing*: Proactively identifies weaknesses before attackers do.

= Legal and Ethical Considerations

Hacking without authorisation is a criminal offence in most countries. In the United States, the Computer Fraud and Abuse Act (CFAA) governs unauthorised computer access. The UK has the Computer Misuse Act. The European Union addresses cybercrime through the Directive on Attacks Against Information Systems.

Ethical hackers must always operate within a clearly defined scope of authorisation, typically documented in a Rules of Engagement (RoE) agreement or a formal penetration testing contract. Responsible disclosure practices dictate that discovered vulnerabilities should be reported to the affected vendor before public disclosure, giving them time to issue a patch — commonly a 90-day window, as popularised by Google's Project Zero.

= Notable Hacking Incidents

- *The Morris Worm (1988)*: One of the first recognised internet worms, released by Robert Morris and causing widespread disruption. Morris was the first person convicted under the CFAA.
- *Operation Aurora (2009–2010)*: A series of sophisticated cyberattacks originating from China targeting Google and dozens of other major corporations, aimed at stealing intellectual property.
- *Stuxnet (2010)*: A highly sophisticated worm, widely attributed to the US and Israel, that physically damaged Iranian nuclear centrifuges — the first known cyberweapon to cause physical destruction.
- *Equifax Breach (2017)*: Exploitation of an Apache Struts vulnerability exposed the personal data of approximately 147 million people.
- *SolarWinds Attack (2020)*: A supply chain compromise allowed attackers to distribute malicious updates to thousands of organisations, including US government agencies.

= Conclusion

Hacking is a multifaceted discipline sitting at the intersection of technical skill, creativity, and ethics. As systems grow more interconnected and critical infrastructure becomes increasingly dependent on software, the importance of understanding offensive techniques — and defending against them — continues to grow. For organisations building systems that handle sensitive data, embedding a security-first mindset from the earliest stages of development is no longer optional; it is a fundamental engineering requirement.
