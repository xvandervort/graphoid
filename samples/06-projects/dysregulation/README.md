# The Dysregulation Model: Addiction & System Failure

This simulation demonstrates a systems-theory approach to addiction and chronic disease progression. It models how a homeostatic control system can become "dysregulated" when a specific component (System Integrity) is degraded by the very stressors it is trying to manage.

## The Theory

In a healthy system (Homeostasis):
1. A **Stressor** pushes a **State** (e.g., dopamine) away from equilibrium.
2. A **Controller** detects this error and applies a counter-force (e.g., receptor downregulation/tolerance).
3. When the Stressor is removed, the Controller reverses its adaptation, returning the system to baseline.

In a dysregulated system (Pathology):
1. The **Stressor** not only affects the State but also causes cumulative damage to **System Integrity**.
2. **System Integrity** determines the *plasticity* or *responsiveness* of the Controller.
3. As Integrity drops, the Controller becomes "stiff" or "laggy."
4. If Integrity drops below a critical tipping point, the Controller cannot reset itself after the Stressor is removed. The system enters a runaway state (withdrawal/crash) that it cannot recover from.

## The Graph Model

The system is modeled as a directed graph where nodes compute their next state based on inputs:

*   `Stressor` -> (+) `State`
*   `State` -> (+) `Controller`
*   `Controller` -> (-) `State` (Negative Feedback Loop)
*   `Stressor` -> (-) `Integrity`
*   `Integrity` -> (modulates) `Controller`

## Running the Simulation

```bash
gr samples/06-projects/dysregulation/main.gr
```

This will output ASCII plots comparing a Resilient System vs. a Dysregulated System.

## Future: Interactive Web Interface

We are planning an interactive web-based version of this simulation, scheduled for **Phase 18.6** of the roadmap.

**Architecture:**
1.  **Backend**: Graphoid running as a local HTTP server (`http.Server`).
2.  **Frontend**: HTML/JS dashboard served by Graphoid.
3.  **Communication**: AJAX calls to `/api/tick` to step through the simulation.

This will allow users to:
*   Adjust "Stress Dose" and "Duration" via sliders.
*   See real-time charts of State vs. Regulation.
*   Experiment with "Integrity Recovery Rate" to see how hard it is to heal a damaged system.
