# METAL Test Server

The **METAL Test Server** is a lightweight backend environment designed for testing and validating the endpoints used by the **METAL Terminal** and the **METAL Website**.  

Unlike the production system, this server **does not implement full security measures**, **does not process real payments**, and should **never be connected to real customer or bank data**. Its sole purpose is to provide developers and testers with a safe sandbox to explore the behavior of METAL APIs, simulate requests, and verify integrations before going live.

---

## Purpose

- ✅ Allow developers to **test and debug API calls** without risk.  
- ✅ Provide a **mock backend** for the METAL Terminal and Website during development.  
- ✅ Serve as a **demo environment** to understand how the METAL ecosystem works.  
- ✅ Enable testers to **experiment freely** without worrying about security or production stability.  

This test server exists so the METAL engineering team can move faster, try new features, and validate assumptions before deploying updates to production.

---

## What It Is Not

- ❌ Not secure — do **not** use it for sensitive data.  
- ❌ Not a real banking system — no real payments are processed.  
- ❌ Not connected to production rails — this is an **isolated environment only**.  
- ❌ Not guaranteed to be stable — endpoints may change as features are being tested.  



---

## Key Features

- **Bank Simulation**: Register mock banks and generate placeholder API keys.  
- **Merchant Simulation**: Add test merchants and link them to simulated devices.  
- **Transaction Endpoints**: Test payment initiation, confirmation, and failure flows.  
- **Website API Testing**: Ensure website dashboards and merchant portals can communicate correctly.  
