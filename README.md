<div align="center">

# MedSup Dash

<p>
  <a href="https://vuejs.org/"><img src="https://img.shields.io/badge/Vue-3.5+-4FC08D?logo=vue.js&logoColor=white" alt="Vue"></a>
  <a href="https://www.typescriptlang.org/"><img src="https://img.shields.io/badge/TypeScript-5.9+-3178C6?logo=typescript&logoColor=white" alt="TypeScript"></a>
  <a href="https://vitejs.dev/"><img src="https://img.shields.io/badge/Vite-7.2+-646CFF?logo=vite&logoColor=white" alt="Vite"></a>
  <a href="https://tailwindcss.com/"><img src="https://img.shields.io/badge/Tailwind_CSS-4.1+-06B6D4?logo=tailwindcss&logoColor=white" alt="Tailwind"></a>
  <a href="https://bun.sh"><img src="https://img.shields.io/badge/Bun-1.1+-000000?logo=bun&logoColor=white" alt="Bun"></a>
</p>

<p>
  <a href="https://github.com/pharmacist-sabot/medsup-dash/actions/workflows/ci.yml"><img src="https://github.com/pharmacist-sabot/medsup-dash/actions/workflows/ci.yml/badge.svg" alt="CI Status"></a>
  <a href="https://github.com/pharmacist-sabot/medsup-dash/blob/main/LICENSE"><img src="https://img.shields.io/github/license/pharmacist-sabot/medsup-dash" alt="License"></a>
  <a href="https://github.com/semantic-release/semantic-release"><img src="https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg" alt="Semantic Release"></a>
  <a href="https://medsup-dash.netlify.app"><img src="https://img.shields.io/badge/Demo-Live-00C7B7?logo=netlify&logoColor=white" alt="Live Demo"></a>
</p>

</div>

---

**MedSup Dash** is a modern, high-performance dashboard application designed for medical supply management. Built with the latest web technologies, it provides a seamless and responsive user experience for tracking key performance indicators (KPIs), visualizing data, and managing inventory.

## 🚀 Tech Stack

This project leverages a cutting-edge stack to ensure speed, scalability, and developer experience:

- **Framework:** [Vue 3](https://vuejs.org/) (Composition API)
- **Build Tool:** [Vite 7](https://vitejs.dev/)
- **Styling:** [Tailwind CSS 4](https://tailwindcss.com/)
- **State Management:** [Pinia](https://pinia.vuejs.org/)
- **Backend / Auth:** [Supabase](https://supabase.com/)
- **Charts:** [Chart.js](https://www.chartjs.org/) & [vue-chartjs](https://vue-chartjs.org/)
- **Icons:** [Lucide Vue](https://lucide.dev/guide/packages/lucide-vue-next)
- **Testing:** [Vitest](https://vitest.dev/)
- **Linting:** [ESLint](https://eslint.org/) (with Antfu config)

## ✨ Key Features

- **🔐 Secure Authentication**: Robust login and session management powered by Supabase Auth.
- **📊 Interactive Dashboard**: Real-time data visualization with dynamic charts and graphs.
- **📈 KPI Tracking**: specialized KPI cards for monitoring critical metrics at a glance.
- **📱 Fully Responsive**: Mobile-first design ensuring perfect usability across all devices.
- **⚡ Blazing Fast**: Optimized build and HMR (Hot Module Replacement) with Vite.
- **🛡️ Type Safe**: Full TypeScript support for reliable and maintainable code.

## 🛠️ Getting Started

Follow these steps to set up the project locally.

### Prerequisites

- **Node.js** (Latest LTS recommended)
- **Bun** (Recommended) or npm/yarn/pnpm

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/pharmacist-sabot/medsup-dash.git
   cd medsup-dash
   ```

2. **Install dependencies**

   ```bash
   bun install
   # or
   npm install
   ```

3. **Environment Setup**

   Create a `.env` file in the root directory by copying the example:

   ```bash
   cp .env.example .env
   ```

   Update `.env` with your Supabase credentials:

   ```env
   VITE_SUPABASE_URL=your_supabase_url
   VITE_SUPABASE_ANON_KEY=your_supabase_anon_key
   ```

4. **Run the development server**

   ```bash
   bun dev
   # or
   npm run dev
   ```

   Open [http://localhost:5173](http://localhost:5173) in your browser.

## 📜 Scripts

| Script          | Description                            |
| :-------------- | :------------------------------------- |
| `dev`           | Start the development server           |
| `build`         | Type-check and build for production    |
| `preview`       | Preview the production build locally   |
| `lint`          | Lint files with ESLint                 |
| `lint:fix`      | Fix linting errors automatically       |
| `test:unit`     | Run unit tests with Vitest             |
| `test:coverage` | Run tests and generate coverage report |

## 📂 Project Structure

```text
medsup-dash/
├── .github/            # GitHub Actions & workflows
├── src/
│   ├── assets/         # Static assets and global styles
│   ├── components/     # Reusable Vue components
│   │   ├── common/     # Generic UI components
│   │   └── dashboard/  # Dashboard-specific widgets
│   ├── layouts/        # App layouts (Default, Blank)
│   ├── router/         # Vue Router configuration
│   ├── services/       # API services (Supabase)
│   ├── stores/         # Pinia state stores
│   ├── types/          # TypeScript type definitions
│   ├── utils/          # Helper functions
│   └── views/          # Page views (Auth, Dashboard)
├── tests/              # Unit tests
├── .env.example        # Environment variables template
├── index.html          # Entry HTML file
├── package.json        # Project dependencies and scripts
├── tailwind.config.js  # Tailwind CSS configuration
└── vite.config.ts      # Vite configuration
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the [MIT License](LICENSE).
