#set document(
  title: "Computers: From Transistors to Modern Computing",
  author: "Lerpz Example Documents",
  keywords: ("computers", "hardware", "software", "computing", "CPU", "memory", "history"),
)

#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 3cm),
)

#set text(
  font: "New Computer Modern",
  size: 11pt,
  lang: "en",
)

#set heading(numbering: "1.1.")
#set par(justify: true, leading: 0.75em)

#show heading.where(level: 1): it => {
  v(1.2em)
  text(size: 16pt, weight: "bold", it)
  v(0.4em)
}

#show heading.where(level: 2): it => {
  v(0.8em)
  text(size: 13pt, weight: "bold", it)
  v(0.3em)
}

// Title block
#align(center)[
  #text(size: 24pt, weight: "bold")[Computers]
  #v(0.4em)
  #text(size: 13pt, style: "italic", fill: luma(80))[From Transistors to Modern Computing]
  #v(0.2em)
  #text(size: 10pt, fill: luma(120))[Example Document — Lerpz RAG Test Corpus]
]

#v(1.5em)
#line(length: 100%, stroke: 0.5pt + luma(180))
#v(1.5em)

// Abstract
#block(
  fill: luma(245),
  inset: 14pt,
  radius: 4pt,
  width: 100%,
)[
  #text(weight: "bold")[Abstract. ]
  A computer is a programmable electronic device that accepts data, performs prescribed mathematical and logical operations at high speed, and displays the results. From room-sized vacuum-tube behemoths of the 1940s to today's billion-transistor processors small enough to fit in a pocket, computers have undergone one of the most dramatic evolutionary arcs in human history. This document surveys the fundamental components, historical milestones, and modern concepts that define the discipline of computing, providing rich, factual content suitable for semantic embedding and retrieval-augmented generation evaluation.
]

#v(1em)

= History of Computing

== Mechanical Precursors

Long before electricity, humans sought machines to automate arithmetic. In 1623, Wilhelm Schickard designed what is considered the first mechanical calculator, capable of adding and subtracting six-digit numbers. Blaise Pascal followed in 1642 with the *Pascaline*, a gear-driven adder built to help his father calculate taxes. Gottfried Wilhelm Leibniz improved on Pascal's design in 1673 with a machine that could also multiply and divide.

The most visionary pre-electronic computing idea came from Charles Babbage, who designed the *Difference Engine* (1822) and the more general *Analytical Engine* (1837). The Analytical Engine introduced concepts that presage modern computers: a "mill" (arithmetic logic unit), a "store" (memory), conditional branching, and loops. Ada Lovelace wrote what is regarded as the first algorithm intended for the Analytical Engine, earning her the title of the world's first computer programmer.

== The Electromechanical Era

The early twentieth century brought electromechanical relay-based machines. Konrad Zuse's *Z3* (1941) in Germany was the first fully programmable, freely programmable electromechanical computer, operating in binary floating point. Simultaneously, Alan Turing formalized the theoretical underpinnings of computation with his 1936 paper introducing the *Turing machine* — an abstract model that defines what it means for a function to be computable.

During World War II, the British built *Colossus* (1943) at Bletchley Park to break German Lorenz cipher traffic, while American engineers constructed the *ENIAC* (Electronic Numerical Integrator and Computer, 1945) at the University of Pennsylvania — the first general-purpose electronic digital computer, filling 167 m² and consuming 150 kW of power.

== The Stored-Program Revolution

John von Neumann's 1945 draft report on the *EDVAC* articulated the stored-program architecture: programs and data share the same memory, instructions are fetched sequentially, and a central processing unit interprets them. This *von Neumann architecture* remains the dominant paradigm today.

The first computers to implement this model were the Manchester Baby (1948) and Cambridge's EDSAC (1949). Commercial production began with the UNIVAC I (1951), which famously predicted the outcome of the 1952 U.S. presidential election on live television.

== Transistors and Integrated Circuits

The invention of the point-contact transistor at Bell Labs in 1947 (Bardeen, Brattain, and Shockley) made vacuum tubes obsolete. Transistors are smaller, faster, more reliable, and far more energy-efficient. By the late 1950s, transistor-based computers like the IBM 7090 replaced their vacuum-tube predecessors.

Jack Kilby (Texas Instruments) and Robert Noyce (Fairchild Semiconductor) independently invented the *integrated circuit* in 1958–1959, placing multiple transistors on a single piece of semiconductor material. Gordon Moore observed in 1965 that the number of transistors on a chip doubled approximately every two years — *Moore's Law* — a trend that held remarkably well for over five decades.

The Intel 4004 (1971) was the first commercially available single-chip microprocessor, integrating 2,300 transistors on a 10 µm process. By contrast, Apple's M4 chip (2024) integrates approximately 28 billion transistors on a 3 nm process.

= Core Hardware Components

== Central Processing Unit (CPU)

The CPU is the brain of the computer, responsible for executing instructions. Its main sub-units are:

- *Arithmetic Logic Unit (ALU)*: Performs integer arithmetic (addition, subtraction, multiplication, division) and bitwise logical operations (AND, OR, XOR, NOT).
- *Control Unit (CU)*: Fetches instructions from memory, decodes them, and orchestrates the activities of all other components.
- *Registers*: Tiny, ultra-fast storage locations within the CPU (e.g., program counter, stack pointer, general-purpose registers).
- *Cache*: Multiple levels (L1, L2, L3) of increasingly large but slower SRAM that buffer frequently accessed data between registers and main memory.

Modern CPUs execute instructions through a pipeline — overlapping fetch, decode, execute, memory-access, and write-back stages to improve throughput. Superscalar designs issue multiple instructions per clock cycle. Branch predictors speculatively execute future instructions to keep pipelines full.

Clock speed (measured in GHz) is one performance metric, but instructions per clock (IPC) and core count matter equally. A contemporary consumer CPU might run at 5 GHz with 24 cores, each capable of executing four instructions per cycle simultaneously.

== Memory Hierarchy

Computers use a hierarchy of memory technologies, trading off speed, capacity, and cost:

+ *Registers* — sub-nanosecond access, a few hundred bytes per core.
+ *L1 Cache* — ~1 ns, 32–512 KB per core, SRAM.
+ *L2 Cache* — ~5 ns, 256 KB–16 MB per core, SRAM.
+ *L3 Cache* — ~20 ns, 8–192 MB shared, SRAM.
+ *Main Memory (RAM)* — ~60–100 ns, 8–256 GB typical, DRAM.
+ *NVMe SSD* — ~100 µs, 500 GB–8 TB typical.
+ *SATA SSD / HDD* — ~0.1–10 ms, 1–20 TB typical.
+ *Optical / Tape* — seconds, virtually unlimited archival.

*Dynamic RAM (DRAM)* stores each bit as a charge in a capacitor that must be refreshed thousands of times per second. DDR5 (Double Data Rate 5), current as of 2024, transfers data at up to 6400 MT/s (megatransfers per second) and supports per-channel capacities of up to 128 GB.

== Storage

Solid-state drives store data in NAND flash cells arranged in pages and blocks. Modern QLC (quad-level cell) NAND stores four bits per cell, pushing capacities up while driving costs below \$0.05 per gigabyte. NVMe drives communicate over PCIe, achieving sequential read speeds exceeding 7,000 MB/s.

Hard disk drives (HDDs) use magnetic platters spinning at 5,400–15,000 RPM with read/write heads that float nanometres above the surface. Despite slower random access, HDDs remain cost-effective for bulk cold storage at \$0.01–0.02 per gigabyte.

== Graphics Processing Unit (GPU)

Originally designed for rendering 3D graphics, GPUs contain thousands of smaller, simpler cores optimized for parallel floating-point arithmetic. NVIDIA's H100 Hopper GPU contains 80 billion transistors and 16,896 CUDA cores, delivering 3,958 TFLOPS of FP8 tensor performance — making it the workhorse of modern AI training workloads.

GPUs communicate with the CPU over PCIe or, in high-end workstations and servers, via NVLink with up to 900 GB/s bidirectional bandwidth.

== Motherboard, Chipset, and Buses

The *motherboard* is the main circuit board interconnecting all components. The *chipset* manages communication between the CPU, RAM, storage, and peripherals. Modern platforms use the PCIe (Peripheral Component Interconnect Express) bus, with PCIe 5.0 offering 32 GT/s per lane (approximately 4 GB/s), giving a ×16 slot a theoretical bandwidth of 64 GB/s.

= Software and Operating Systems

== Layers of Abstraction

Computer software is organized in layers of abstraction:

1. *Firmware / BIOS / UEFI*: Low-level code stored in ROM that initializes hardware and boots the operating system.
2. *Operating System Kernel*: Manages hardware resources — CPU scheduling, memory management, device drivers, filesystems.
3. *System Libraries and APIs*: POSIX, Win32, libc — interfaces that applications call instead of interacting with the kernel directly.
4. *Runtime Environments*: JVM, CLR, Python interpreter — abstract platforms for higher-level languages.
5. *Application Software*: Web browsers, databases, text editors, games.

== Operating Systems

An operating system (OS) is the software layer that abstracts hardware for applications. Core responsibilities include:

- *Process management*: Creating, scheduling, and terminating processes and threads.
- *Memory management*: Virtual memory, paging, demand loading, and protection between processes.
- *Filesystem management*: Hierarchical namespace, permissions, journaling (ext4, APFS, NTFS, ZFS).
- *Device drivers*: Kernel modules that translate generic I/O calls into device-specific commands.
- *Networking*: TCP/IP stack, socket API, firewall integration.

Major operating systems include Linux (kernel first released 1991 by Linus Torvalds), Windows NT (1993, Microsoft), macOS/Darwin (Apple, XNU kernel), and Android/iOS (mobile, Linux and Darwin kernels respectively).

== Programming Languages and Compilation

Programs are written in high-level languages and translated to machine code through compilation or interpretation:

- *Compiled languages* (C, C++, Rust, Go): Source code is translated ahead-of-time into native machine instructions, producing fast binaries.
- *Interpreted languages* (Python, Ruby): Source is executed line-by-line by an interpreter at runtime, trading speed for flexibility.
- *JIT-compiled languages* (Java, C\#, JavaScript via V8): Bytecode is compiled to native code at runtime, balancing portability and performance.

Rust, introduced by Mozilla in 2010 and reaching 1.0 in 2015, combines C-level performance with memory-safety guarantees enforced at compile time through its *ownership and borrow checker* system, making it increasingly popular for systems programming.

= Networking and the Internet

Computers communicate through networks governed by layered protocol stacks. The *Internet Protocol Suite* (TCP/IP) organizes communication into four conceptual layers:

+ *Link layer*: Ethernet (IEEE 802.3), Wi-Fi (IEEE 802.11), fiber, copper — physical transmission.
+ *Internet layer*: IP (IPv4, IPv6) — global addressing and routing.
+ *Transport layer*: TCP (reliable, ordered), UDP (fast, unreliable), QUIC (modern, encrypted).
+ *Application layer*: HTTP/3, DNS, SMTP, SSH, TLS.

The World Wide Web, invented by Tim Berners-Lee at CERN in 1989, is an application layer built atop the Internet using HTTP and HTML. As of 2024, approximately 5.4 billion people — over 67% of the global population — use the Internet.

= Modern Computing Paradigms

== Cloud Computing

Cloud computing delivers on-demand computing resources — servers, storage, databases, networking, analytics — over the Internet on a pay-as-you-go basis. Major providers include Amazon Web Services (AWS, launched 2006), Microsoft Azure (2010), and Google Cloud Platform (GCP, 2011).

Cloud services are categorized as:
- *IaaS* (Infrastructure as a Service): Raw virtual machines and storage (EC2, GCE).
- *PaaS* (Platform as a Service): Managed runtimes and databases (Heroku, Cloud Run).
- *SaaS* (Software as a Service): Complete applications delivered over the web (Gmail, Salesforce).

== Artificial Intelligence and Machine Learning

Modern computers running GPUs and specialized AI accelerators (Google TPU, Apple Neural Engine) have made large-scale machine learning practical. *Deep learning* — training multi-layer artificial neural networks on massive datasets — underpins breakthroughs in image recognition, natural language processing, and generative AI.

Large language models (LLMs) such as GPT-4 contain hundreds of billions of parameters and require clusters of thousands of GPUs and petabytes of training data to develop. Inference — running a trained model — can happen on devices as small as a smartphone thanks to quantization (reducing weight precision from FP32 to INT4).

== Quantum Computing

Quantum computers exploit quantum mechanical phenomena — superposition and entanglement — to perform certain computations exponentially faster than classical computers. Unlike classical bits, *qubits* can represent 0, 1, or any quantum superposition of both simultaneously.

Algorithms such as Shor's algorithm (integer factorization) and Grover's algorithm (unstructured search) offer theoretical speedups. As of 2024, IBM's Condor processor has 1,121 qubits, though noise and decoherence remain significant engineering challenges before quantum advantage is demonstrated for practically useful problems at scale.

= Key Facts Summary

#block(
  fill: luma(245),
  inset: 14pt,
  radius: 4pt,
  width: 100%,
)[
  - The first general-purpose electronic computer, ENIAC (1945), weighed 27 tonnes and occupied 167 m².
  - Moore's Law predicted transistor counts doubling every ~2 years; the Apple M4 contains 28 billion transistors.
  - Modern CPUs operate at up to ~5 GHz with up to 96+ consumer cores.
  - DDR5 RAM transfers data at up to 6,400 MT/s per channel.
  - NVMe SSDs achieve sequential reads exceeding 7,000 MB/s via PCIe 5.0.
  - GPUs power modern AI: the NVIDIA H100 delivers 3,958 TFLOPS of FP8 tensor performance.
  - The Linux kernel, started by Linus Torvalds in 1991, now contains over 27 million lines of code.
  - Quantum computers with 1,000+ qubits exist, but practical quantum advantage remains an open research goal.
  - Over 5.4 billion people use the Internet as of 2024.
]

#v(2em)
#align(center)[
  #text(size: 9pt, fill: luma(150))[
    This document is part of the Lerpz example corpus for RAG and semantic embedding evaluation. \
    All information is factual and intended for testing retrieval quality against computing-domain queries.
  ]
]
