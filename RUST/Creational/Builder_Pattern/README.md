# Voyagr — AI Travel Assistant (Claude/Perplexity-style)

A React frontend where conversation is the whole product. It opens as a
full-width chat (like Claude's landing screen); the moment the backend
returns destination photos, a preview panel slides in from the left —
weather, a photo gallery, and the assistant's description — while the chat
compresses to the right, like a mirrored Claude Artifact.

## Backend response shape

`src/api/api.js` → `sendChatMessage()` talks to the real `/chat` shape:

```json
{
  "text": "AI generated response",
  "used_web_search": true,
  "photos": [
    { "url": "https://...", "description": "..." },
    { "url": "https://...", "description": null }
  ],
  "weather": { "summary": "broken clouds", "temp_c": 28.3, "source": "OpenWeatherMap" }
}
```

`photos` is treated as a list of *candidates* — some of the URLs (scraped
from web search results) 404, hotlink-block, or otherwise fail to load.
Rather than trying to pre-validate them on the backend, each `<PhotoCard>`
just attempts to load its own image and quietly renders nothing if it fails,
so the grid only ever shows photos that are actually present. Everything
still funnels through `sendChatMessage()`, so if the shape changes again,
that's the one place to adjust.

## Preview panel

- **Weather** sits at the top of the panel with a summary-driven emoji icon
  and the temperature (`WeatherCard`).
- **Photos** render as a two-column card grid (`PhotoGallery` /
  `PhotoCard`); each card shows the image with its description overlaid at
  the bottom, when one was provided.
- Clicking a card opens a **lightbox modal** (`PhotoModal`) with the image
  on top and the full description underneath at subtitle size.
- A **cross button** in the panel header (`closePreview` in `AppContext`)
  lets the user dismiss the whole section without losing the chat thread
  underneath it.
- The panel only slides in — and the chat column smoothly animates
  narrower to make room — once the backend actually returns at least one
  photo candidate (`Home.jsx` / `AppContext.jsx`, `LAYOUT.PREVIEW_OPEN`).

## Background artwork

The warm watercolor travel illustration lives behind the whole app
(`src/css/global.css`, on `body`), tinted with a translucent cream overlay so
text and cards stay readable. It swaps to a portrait-oriented version below
780px (`public/backgrounds/travel-hero-mobile.jpg`) so the composition still
works on phones instead of just cropping the landscape version awkwardly.
Both images were compressed to JPG (~130-150KB each, down from ~2MB PNGs) so
they don't slow down first load, especially on mobile data.

## Mobile responsiveness

- Navbar collapses link labels to icons under 640px, and drops the logo text
  entirely under 380px so it doesn't overflow on small phones
- Chat bubbles widen to 90% of the container on small screens instead of
  staying capped at 78%
- The preview panel gets a **collapse toggle** on tablet/mobile (`Home.jsx`) —
  since the panel stacks above the chat there instead of sitting beside it,
  users can collapse it down to a thin header bar to get more room for the
  conversation, then tap to expand it again
- Uses `100dvh` (with a `100vh` fallback) instead of plain `100vh` for
  full-height layout, since `100vh` is unreliable on mobile Safari/Chrome
  when the address bar shows/hides

## Auth

The real backend requires a JWT on `/chat`, so this build includes a full
register/login flow:

- `src/context/AuthContext.jsx` — stores the JWT in `localStorage`, restores
  the session via `GET /me` on page load, exposes `login`/`register`/`logout`
- `src/api/api.js` — an axios request interceptor attaches
  `Authorization: Bearer <token>` to every request automatically, so
  individual components never touch the token directly
- `/` (the chat) is wrapped in `ProtectedRoute` — visiting it while logged
  out redirects to `/login`
- If `/chat` ever returns a 401 (expired/invalid token), the app logs the
  user out and redirects to `/login` automatically — see the
  `AppProviderWithAuthBridge` wrapper in `App.jsx`

`/auth/register` and `/auth/login` match your backend's actual shape:
register takes JSON `{ email, password }`; login is an OAuth2 password-flow
form (`application/x-www-form-urlencoded`, field name `username`).

## Stack

- React (Vite, JavaScript)
- React Router DOM
- Axios
- Framer Motion (panel/chat transitions, photo modal)
- Lucide React (icons)
- React Markdown + remark-gfm (assistant messages support lists, tables, code blocks, links)

No Tailwind/Bootstrap/UI kits — plain modular CSS per component.

## Docker

```bash
# build (bakes VITE_API_URL into the static bundle at build time)
docker build -t voyagr-ai --build-arg VITE_API_URL=https://your-backend.example.com .

# run
docker run -p 8080:80 voyagr-ai
```

or with Compose:

```bash
VITE_API_URL=https://your-backend.example.com docker compose up --build
```

The image is a two-stage build — `node:20-alpine` builds the Vite bundle,
then `nginx:1.27-alpine` serves the static `dist/` output with gzip and SPA
fallback routing (`nginx.conf`) so client-side routes like `/login` and
`/about` don't 404 on refresh. The app is served on port 80 inside the
container (mapped to `8080` above).

## Run it

```bash
npm install
cp .env.example .env
# edit .env — point VITE_API_URL at your Render backend (or localhost:8000)
npm run dev
```

## How the two UI states work

State lives in `src/context/AppContext.jsx`:

- `layoutState` is either `FULL_CHAT` or `PREVIEW_OPEN`
- Sending a message always appends to `messages` and shows a typing indicator
- The moment a response includes a `photo`, the context stores `photo`,
  `weather`, and `destination`, and flips `layoutState` to `PREVIEW_OPEN`
- If a later response has no photo, the existing preview is left as-is
  (mirrors an Artifact staying open until replaced) — this is a judgment call,
  easy to change in `sendMessage()` if you'd rather it hide immediately

`Home.jsx` reads `layoutState` and animates between the two layouts with
Framer Motion — no page reload, no route change.

## Project structure

```
src/
  api/api.js                  # axios instance + sendChatMessage()
  context/AppContext.jsx      # messages, loading, photo, weather, destination, layoutState
  components/
    Navbar/  ChatWindow/  ChatBubble/  PromptInput/
    PreviewPanel/  WeatherCard/  DestinationImage/  MapCard/
    LoadingSkeleton/  TypingIndicator/  ErrorCard/
  pages/
    Home/    # the two-state chat + preview experience
    About/   # the "Information" nav page
  App.jsx
  main.jsx
```

## Responsive behavior

- **Desktop (>1024px)**: preview 65% / chat 35%, side by side
- **Tablet (≤1024px)**: preview stacks on top, chat below
- **Mobile (≤640px)**: same stacked layout with tighter heights so both stay usable

## Error handling

Network failures, timeouts, and backend errors surface as a dismissible
banner (`ErrorCard`) above the chat rather than a silent failure or a raw
error dump.

## Build

```bash
npm run build     # outputs to dist/
npm run preview   # serve the production build locally
```

Set `VITE_API_URL` to your deployed backend URL before building.
