# Lynix API
Lynix API is an API used by the lynix.ca website and services created in rust.

# Migration Progress
| Feature / Service | Status | Latest Version Available | Note                                       | 
| ------- | ---- |--------------------------|--------------------------------------------|
| Event Management System | ⚠️ | v2.0.X                   | Missing CRUD                               |
| Authentication | ⚠️ | v2.0.X                   | Missing Critical Parts                     |
| Blog System | ❌ | v1.8.X (TS)              | Not Implemented in Rust                    |
| CMS Page System | ❌ | v1.8.X (TS)              | Not Implemented in Rust                    |
| FurSystem | ❌ | v1.8.X (TS)              | Not Implemented in Rust                    |
| WolfHR | ✅ | v2.1.X                   | Can't compile in this commit due to errors |
| Discord Webhooks | ❌ | v1.3.X (TS)              | Not Implemented in Rust                    |
| Websockets | ❌ | v1.0.X (TS)              | Not Implemented in Rust                    |


# Features
- Event Management System
- Blog System
- CMS Page System 
  - Import from Obsidian
- Fursuit (FurSystem) Controller
- WolfHR Built-In for Supported Watches
  - This requires NeosVR or VRChat and a compatible WearOS Watch
- Discord Webhook Support

# Authentication Features
| Feature | Status | Note | 
| ------- | ------ | ---- | 
| Basic Authentication | ✅ | |
| Password Hasing | ✅ | |
| Session / Bearer Tokens / JWT | ❌ | To be implemented |
| TOTP | ❌ | To be implemented |
| Webauthn | ❌ | |
| Account Management | ❌ | To be implemented |
 
