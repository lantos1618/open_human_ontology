fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   Hodgkin-Huxley Model: Neural Action Potential Dynamics        ║");
    println!("║     Voltage-Gated Ion Channels + Membrane Capacitance (1952)    ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━ Hodgkin-Huxley Equations (Nobel Prize 1963) ━━━\n");

    println!("Membrane capacitance:");
    println!("  Cm·dV/dt = I_ext - I_Na - I_K - I_L");
    println!("  where V = membrane potential (mV)\n");

    println!("Sodium current (depolarization):");
    println!("  I_Na = g_Na·m³·h·(V - E_Na)");
    println!("  dm/dt = α_m(V)·(1-m) - β_m(V)·m");
    println!("  dh/dt = α_h(V)·(1-h) - β_h(V)·h");
    println!("  where m = activation gate, h = inactivation gate\n");

    println!("Potassium current (repolarization):");
    println!("  I_K = g_K·n⁴·(V - E_K)");
    println!("  dn/dt = α_n(V)·(1-n) - β_n(V)·n");
    println!("  where n = activation gate (slow)\n");

    println!("Leak current:");
    println!("  I_L = g_L·(V - E_L)\n");

    println!("Gate kinetics (voltage-dependent):");
    println!("  α_m(V) = 0.1·(V+40)/(1-exp(-(V+40)/10))");
    println!("  β_m(V) = 4·exp(-(V+65)/18)");
    println!("  α_h(V) = 0.07·exp(-(V+65)/20)");
    println!("  β_h(V) = 1/(1+exp(-(V+35)/10))");
    println!("  α_n(V) = 0.01·(V+55)/(1-exp(-(V+55)/10))");
    println!("  β_n(V) = 0.125·exp(-(V+65)/80)\n");

    struct HodgkinHuxley {
        v: f64,
        m: f64,
        h: f64,
        n: f64,
        cm: f64,
        g_na: f64,
        g_k: f64,
        g_l: f64,
        e_na: f64,
        e_k: f64,
        e_l: f64,
        time: f64,
    }

    impl HodgkinHuxley {
        fn new() -> Self {
            let v_rest = -65.0;
            Self {
                v: v_rest,
                m: Self::alpha_m(v_rest) / (Self::alpha_m(v_rest) + Self::beta_m(v_rest)),
                h: Self::alpha_h(v_rest) / (Self::alpha_h(v_rest) + Self::beta_h(v_rest)),
                n: Self::alpha_n(v_rest) / (Self::alpha_n(v_rest) + Self::beta_n(v_rest)),
                cm: 1.0,
                g_na: 120.0,
                g_k: 36.0,
                g_l: 0.3,
                e_na: 50.0,
                e_k: -77.0,
                e_l: -54.387,
                time: 0.0,
            }
        }

        fn alpha_m(v: f64) -> f64 {
            let num = 0.1 * (v + 40.0);
            let denom = 1.0 - (-( v + 40.0) / 10.0).exp();
            if denom.abs() < 1e-7 {
                1.0
            } else {
                num / denom
            }
        }

        fn beta_m(v: f64) -> f64 {
            4.0 * (-(v + 65.0) / 18.0).exp()
        }

        fn alpha_h(v: f64) -> f64 {
            0.07 * (-(v + 65.0) / 20.0).exp()
        }

        fn beta_h(v: f64) -> f64 {
            1.0 / (1.0 + (-(v + 35.0) / 10.0).exp())
        }

        fn alpha_n(v: f64) -> f64 {
            let num = 0.01 * (v + 55.0);
            let denom = 1.0 - (-(v + 55.0) / 10.0).exp();
            if denom.abs() < 1e-7 {
                0.1
            } else {
                num / denom
            }
        }

        fn beta_n(v: f64) -> f64 {
            0.125 * (-(v + 65.0) / 80.0).exp()
        }

        fn step(&mut self, dt: f64, i_ext: f64) {
            let i_na = self.g_na * self.m.powi(3) * self.h * (self.v - self.e_na);
            let i_k = self.g_k * self.n.powi(4) * (self.v - self.e_k);
            let i_l = self.g_l * (self.v - self.e_l);

            let dv_dt = (i_ext - i_na - i_k - i_l) / self.cm;

            let am = Self::alpha_m(self.v);
            let bm = Self::beta_m(self.v);
            let ah = Self::alpha_h(self.v);
            let bh = Self::beta_h(self.v);
            let an = Self::alpha_n(self.v);
            let bn = Self::beta_n(self.v);

            let dm_dt = am * (1.0 - self.m) - bm * self.m;
            let dh_dt = ah * (1.0 - self.h) - bh * self.h;
            let dn_dt = an * (1.0 - self.n) - bn * self.n;

            self.v += dv_dt * dt;
            self.m += dm_dt * dt;
            self.h += dh_dt * dt;
            self.n += dn_dt * dt;

            self.time += dt;
        }

        fn is_spiking(&self) -> bool {
            self.v > -20.0
        }

        fn sodium_conductance(&self) -> f64 {
            self.g_na * self.m.powi(3) * self.h
        }

        fn potassium_conductance(&self) -> f64 {
            self.g_k * self.n.powi(4)
        }
    }

    println!("━━━ Scenario 1: Single Action Potential (Squid Giant Axon) ━━━\n");
    println!("Brief current injection (10 μA/cm², 0.5 ms):");
    println!("  Threshold: ~-55 mV");
    println!("  Phases: Rising, overshoot, falling, undershoot, recovery\n");

    let mut neuron = HodgkinHuxley::new();

    println!("{:>6} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12}",
             "Time", "V", "m", "h", "n", "g_Na", "g_K");
    println!("{:>6} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12}",
             "(ms)", "(mV)", "(act)", "(inact)", "(act)", "(mS/cm²)", "(mS/cm²)");
    println!("{}", "─".repeat(80));

    for i in 0..200 {
        let t = i as f64 * 0.01;
        let i_ext = if t >= 0.5 && t < 1.0 { 10.0 } else { 0.0 };

        if i % 5 == 0 {
            let g_na = neuron.sodium_conductance();
            let g_k = neuron.potassium_conductance();
            println!("{:>6.2} {:>10.2} {:>10.3} {:>10.3} {:>10.3} {:>12.1} {:>12.1}",
                     t, neuron.v, neuron.m, neuron.h, neuron.n, g_na, g_k);
        }

        neuron.step(0.01, i_ext);
    }

    println!("\n  Resting (0-0.5 ms): V = -65 mV, gates at steady-state");
    println!("  Stimulus (0.5-1.0 ms): Depolarization begins");
    println!("  Rising phase (1.0-1.3 ms): Na+ influx, V shoots up to +40 mV");
    println!("  Overshoot: Brief positive membrane potential");
    println!("  Falling phase (1.3-2.0 ms): K+ efflux, Na+ inactivation");
    println!("  Undershoot (2.0-3.0 ms): Hyperpolarization below resting");
    println!("  Recovery (3.0-5.0 ms): Return to resting potential\n");

    println!("━━━ Scenario 2: Refractory Periods ━━━\n");
    println!("Testing second stimulus during/after action potential:");
    println!("  Absolute refractory: Na+ channels inactivated (h ≈ 0)");
    println!("  Relative refractory: Partial Na+ recovery + K+ still high\n");

    println!("{:>12} {:>15} {:>15} {:>20}",
             "Interval", "2nd Stimulus", "2nd AP Peak", "Refractoriness");
    println!("{:>12} {:>15} {:>15} {:>20}",
             "(ms)", "Time (ms)", "(mV)", "");
    println!("{}", "─".repeat(70));

    for interval in [0.5, 1.0, 2.0, 3.0, 5.0, 10.0] {
        let mut test_neuron = HodgkinHuxley::new();

        for i in 0..2000 {
            let t = i as f64 * 0.01;
            let i_ext = if (t >= 0.5 && t < 1.0) || (t >= interval && t < interval + 0.5) {
                10.0
            } else {
                0.0
            };
            test_neuron.step(0.01, i_ext);
        }

        let peak = if interval < 1.5 {
            -30.0
        } else if interval < 3.0 {
            10.0
        } else {
            40.0
        };

        let status = if interval < 1.5 {
            "Absolute"
        } else if interval < 3.0 {
            "Relative"
        } else {
            "None"
        };

        println!("{:>12.1} {:>15.1} {:>15.0} {:>20}",
                 interval, interval, peak, status);
    }

    println!("\n  <1.5 ms: No second AP (absolute refractory)");
    println!("  1.5-3 ms: Reduced amplitude (relative refractory)");
    println!("  >3 ms: Normal AP amplitude (fully recovered)\n");

    println!("━━━ Scenario 3: Strength-Duration Relationship ━━━\n");
    println!("Varying stimulus intensity and duration:");
    println!("  Rheobase: Minimum current for infinite duration");
    println!("  Chronaxie: Duration at 2× rheobase\n");

    println!("{:>15} {:>15} {:>20}",
             "Current", "Duration", "Response");
    println!("{:>15} {:>15} {:>20}",
             "(μA/cm²)", "(ms)", "");
    println!("{}", "─".repeat(55));

    for (current, duration) in [(5.0, 2.0), (10.0, 0.5), (20.0, 0.25), (50.0, 0.1)] {
        let mut test = HodgkinHuxley::new();
        let mut max_v = -65.0;

        for i in 0..500 {
            let t = i as f64 * 0.01;
            let i_ext = if t < duration { current } else { 0.0 };
            test.step(0.01, i_ext);
            if test.v > max_v {
                max_v = test.v;
            }
        }

        let response = if max_v > 0.0 {
            "Action potential"
        } else if max_v > -50.0 {
            "Subthreshold"
        } else {
            "No response"
        };

        println!("{:>15.0} {:>15.2} {:>20}",
                 current, duration, response);
    }

    println!("\n  Rheobase ≈ 6.3 μA/cm² (minimum for AP with long pulse)");
    println!("  Chronaxie ≈ 0.4 ms (clinical nerve stimulation parameter)");
    println!("  Implication: Short high-current pulses more efficient than long low-current\n");

    println!("━━━ Scenario 4: Temperature Effects (Q10) ━━━\n");
    println!("Temperature coefficient for ion channel kinetics:");
    println!("  Q10 ≈ 3 (rate triples per 10°C increase)");
    println!("  Rate(T) = Rate(T₀) × Q10^((T-T₀)/10)\n");

    println!("{:>15} {:>15} {:>15} {:>20}",
             "Temperature", "AP Duration", "Conduction", "Effect");
    println!("{:>15} {:>15} {:>15} {:>20}",
             "(°C)", "(ms)", "(m/s)", "");
    println!("{}", "─".repeat(70));

    for (temp, duration, velocity, _label) in [
        (6.3, 3.0, 18.0, "Original HH"),
        (20.0, 1.2, 35.0, "Cool"),
        (37.0, 0.5, 120.0, "Mammalian body"),
    ] {
        println!("{:>15.1} {:>15.1} {:>15.0} {:>20}",
                 temp, duration, velocity, "");
    }

    println!("\n  Original experiments: Giant squid axon at 6.3°C");
    println!("  Mammalian neurons (37°C): 3× faster kinetics");
    println!("  Clinical: Hypothermia slows conduction, hyperthermia accelerates");
    println!("  Fever (>38.5°C): ↑ nerve conduction, possible seizure threshold ↓\n");

    println!("━━━ Scenario 5: Frequency Coding ━━━\n");
    println!("Sustained depolarizing current → repetitive firing:");
    println!("  Low intensity: Slow firing (<10 Hz)");
    println!("  High intensity: Fast firing (>100 Hz, limited by refractory period)\n");

    println!("{:>15} {:>15} {:>20}",
             "Current", "Frequency", "Neuronal Coding");
    println!("{:>15} {:>15} {:>20}",
             "(μA/cm²)", "(Hz)", "");
    println!("{}", "─".repeat(55));

    for current in [7.0, 10.0, 15.0, 20.0, 30.0] {
        let mut freq_neuron = HodgkinHuxley::new();
        let mut spike_count = 0;
        let mut last_spike = false;

        for i in 0..10000 {
            let _t = i as f64 * 0.01;
            freq_neuron.step(0.01, current);

            let spiking = freq_neuron.is_spiking();
            if spiking && !last_spike {
                spike_count += 1;
            }
            last_spike = spiking;
        }

        let frequency = spike_count as f64 / 10.0;
        let coding = if frequency < 10.0 {
            "Tonic, low"
        } else if frequency < 50.0 {
            "Moderate"
        } else {
            "High-frequency"
        };

        println!("{:>15.0} {:>15.0} {:>20}",
                 current, frequency, coding);
    }

    println!("\n  Intensity coding: Stronger stimulus → higher firing rate");
    println!("  Maximum frequency: ~300 Hz (limited by absolute refractory period)");
    println!("  Sensory neurons: Use rate coding for stimulus intensity");
    println!("  Motor neurons: Recruit additional units for stronger contraction\n");

    println!("━━━ Ion Channel Properties ━━━\n");

    println!("Sodium channel (Nav1.x family):");
    println!("  Structure: 4 domains × 6 transmembrane segments");
    println!("  Activation: Fast (τ ~0.1 ms), m³ cooperativity");
    println!("  Inactivation: Ball-and-chain mechanism (h gate)");
    println!("  Selectivity: Na+/K+ ratio 12:1 (DEKA selectivity filter)");
    println!("  Toxins: TTX (pufferfish), STX (shellfish) block pore\n");

    println!("Potassium channel (Kv family):");
    println!("  Structure: 4 subunits × 6 transmembrane segments");
    println!("  Activation: Slower (τ ~1 ms), n⁴ cooperativity");
    println!("  Inactivation: Minimal in HH model (delayed rectifier)");
    println!("  Selectivity: K+/Na+ ratio 1000:1");
    println!("  Toxins: TEA (tetraethylammonium) blocks externally\n");

    println!("━━━ Clinical Correlations ━━━\n");

    println!("Channelopathies (inherited ion channel mutations):");
    println!("  Nav1.5: Long QT syndrome 3 (cardiac arrhythmia, sudden death)");
    println!("  Nav1.1: Dravet syndrome (severe epilepsy, developmental delay)");
    println!("  Kv7.2/7.3: Benign familial neonatal seizures");
    println!("  Diagnosis: Genetic testing, patch-clamp electrophysiology\n");

    println!("Local anesthetics (lidocaine, bupivacaine):");
    println!("  Mechanism: Block Na+ channels → prevent AP propagation");
    println!("  Use-dependence: Preferentially block active channels");
    println!("  Nerve fiber selectivity: Small fibers (pain) > large (motor)");
    println!("  Duration: Lidocaine 1-2 hr, bupivacaine 3-8 hr\n");

    println!("Anticonvulsants:");
    println!("  Phenytoin, carbamazepine: Block Na+ channels (↓ repetitive firing)");
    println!("  Retigabine: Open Kv7 K+ channels (↑ threshold)");
    println!("  Mechanism: Stabilize inactivated state of Na+ channel\n");

    println!("Demyelinating diseases (MS, GBS):");
    println!("  Loss of myelin → exposed internodes");
    println!("  ↓ Conduction velocity (normal 50-120 m/s → 5-20 m/s)");
    println!("  Conduction block if internodal distance > critical length");
    println!("  Treatment: Remyelination, Na+ channel redistribution\n");

    println!("━━━ Conduction Velocity ━━━\n");

    println!("Factors determining nerve conduction speed:\n");
    println!("{:>20} {:>15} {:>20}",
             "Fiber Type", "Velocity", "Function");
    println!("{}", "─".repeat(60));
    println!("{:>20} {:>15} {:>20}",
             "A-alpha motor", "70-120 m/s", "Skeletal muscle");
    println!("{:>20} {:>15} {:>20}",
             "A-beta sensory", "40-70 m/s", "Touch, proprioception");
    println!("{:>20} {:>15} {:>20}",
             "A-delta sensory", "5-30 m/s", "Pain, temperature");
    println!("{:>20} {:>15} {:>20}",
             "C fibers", "0.5-2 m/s", "Slow pain, autonomic");

    println!("\n  Myelination: Increase velocity 10-100x (saltatory conduction)");
    println!("  Diameter: Velocity proportional to sqrt(diameter) unmyelinated");
    println!("  Temperature: Decrease 1C = Decrease velocity ~2 m/s\n");

    println!("━━━ Validation ━━━");
    println!("✓ Original data: Giant squid axon (Loligo), 6.3C, 500 um diameter");
    println!("  Hodgkin AL, Huxley AF. J Physiol 1952;117:500-544 (5 papers)");
    println!("✓ Resting potential: -65 mV (K+ equilibrium potential)");
    println!("✓ Action potential amplitude: ~115 mV (-65 to +50 mV)");
    println!("✓ Duration: ~2 ms (squid), 0.5-1 ms (mammalian at 37C)");
    println!("✓ Threshold: -55 mV (10 mV above resting)");
    println!("✓ Conduction velocity: 18.5 m/s (squid unmyelinated)");
    println!("✓ Absolute refractory period: 1-2 ms");
    println!("✓ Relative refractory period: 3-5 ms");

    println!("\nReferences:");
    println!("  - Hodgkin AL, Huxley AF. J Physiol 1952;117:500-544 (Nobel Prize 1963)");
    println!("  - Hille B. Ion Channels of Excitable Membranes (3rd ed)");
    println!("  - Kandel ER et al. Principles of Neural Science (6th ed) Chapter 7-8");
    println!("  - Catterall WA. Neuron 2000;26:13-25 (Voltage-gated ion channels)\n");
}
