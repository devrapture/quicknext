# QuickNext 🚀

QuickNext is a command-line tool written in Rust that helps you bootstrap Next.js projects with modern configurations and best practices. It streamlines the process of setting up a new Next.js project with Tailwind CSS and other commonly used tools.

## Features ✨

- **Next.js App Router Setup**: Automatically configures Next.js with the new App Router
- **Tailwind CSS Integration**: Pre-configured Tailwind CSS setup with PostCSS
- **Code Formatting**: Includes Prettier configuration with Tailwind plugin
- **Git Integration**: Optional Git repository initialization
- **Package Management**: Automated dependency installation and configuration
- **Modern Defaults**: Sets up modern Next.js features and configurations out of the box

## Installation 🛠️

```bash
# Or build from source
git clone https://github.com/devrapture/quicknext.git
cd quicknext
cargo install --path .
```


## Configuration Files 🔧

### Tailwind CSS
QuickNext sets up Tailwind CSS with PostCSS and includes:
- `tailwind.config.js` - Tailwind configuration
- `postcss.config.js` - PostCSS configuration
- Prettier plugin for Tailwind CSS

### Scripts

The generated `package.json` includes useful scripts:

```json
{
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "format:write": "prettier --write \"**/*.{ts,tsx,js,jsx,mdx}\" --cache",
    "format:check": "prettier --check \"**/*.{ts,tsx,js,jsx,mdx}\" --cache"
  }
}
```

## Contributing 🤝

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License 📝

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- [Create t3 app](https://create.t3.gg/)
- [Next.js](https://nextjs.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Rust](https://www.rust-lang.org/)

## Support 💬

If you have any questions or run into issues, please open an issue in the GitHub repository.

---

Built with ❤️ using Rust 