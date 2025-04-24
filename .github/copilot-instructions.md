# SHOPIFY THEME DEVELOPMENT - ASCENT V2.1.8

## CRITICAL GUIDELINES

### CODE STRUCTURE

- File organization: assets(CSS/JS), config, layout, locales, sections, snippets, templates
- Always use kebab-case for filenames
- Component approach: prefix CSS with component name (component-\*.css)
- BEM methodology for CSS classes

### CODE STANDARDS

- **Liquid**: 2-space indent, minimal template logic, use snippets for reuse
- **Array handling**: assign arrays = '' | split: ',', append with concat
- **JS**: ES6+, modular, async/await, proper error handling
- **CSS**: Custom properties, mobile-first, semantic classes

### VERSION CONTROL WORKFLOW

- Pull from development branch before work
- Branch format: GH-{issue-number}\_ShortSummaryName
- Create commit message based on the .github/.gitmessage file

### LOCALIZATION SYSTEM

- Storage: /locales directory
- Key files: en.default.json (frontend), en.default.schema.json (admin)
- Translation pattern: {{ 'key.path' | t }}
- NEVER hard-code strings in schema sections

## HIGH PRIORITY GUIDELINES

### ISSUE MANAGEMENT

- Create labels with: meaningful name, description, appropriate color
- Use issue templates from .github/ISSUE_TEMPLATE/
- Link PRs to original issues

### ACCESSIBILITY

- Follow WCAG 2.1 guidelines
- Implement proper ARIA attributes
- Ensure keyboard navigation
- Maintain color contrast ratios
- Always add alt text to images

### TRANSLATION KEY STRUCTURE

- Sections: sections.<section-name>.<key>
- Settings: sections.<section-name>.settings.<setting-id>.label
- Options: sections.<section-name>.settings.<setting-id>.options\_\_<number>
- Presets: sections.<section-name>.presets.<preset-name>
- Blocks: sections.<section-name>.blocks.<block-type>.name

### PERFORMANCE

- Run theme check before deployment
- Optimize images, minify assets
- Implement lazy loading
- Use critical CSS
- Test performance across devices

## STANDARD PRACTICES

### TESTING WORKFLOW

- Theme Check: validate before commits
- Browser Testing: Chrome, Firefox, Safari, Edge
- Functional Testing: interactive elements, forms, cart
- Accessibility Testing: keyboard, screen readers

### DOCUMENTATION

- Keep documentation updated
- Document theme customizations
- Document custom liquid filters
- Maintain a changelog

### SECURITY

- Validate user inputs
- Escape output appropriately
- Follow Shopify security guidelines
- Regular security audits

### GITHUB LABEL COLORE PALETTE

- Red (Critical): cc0000, d40000, db0000, e20000, e60000
- Orange (Warning): cc6d00, d47200, db7700, e27c00, e68200
- Green (Features): 2e8540, 357a47, 3c6f4f, 436557, 4a5b5e
- Blue (Docs): 0039cc, 0040d4, 0047db, 004ee2, 0055e6
- Purple (UI/UX): 5500cc, 5c00d4, 6300db, 6a00e2, 7100e6
- Teal (Misc): 008080, 207676, 406c6c, 606262, 805858
