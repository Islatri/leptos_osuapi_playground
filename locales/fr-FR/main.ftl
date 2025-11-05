ciallo = bonjour
demo = démo

# Footer content
footer-description = Client API osu! en Rust haute performance, bien structuré et extensible. Prend en charge les environnements WASM et natifs.
footer-docs-heading = Documentation
footer-docs-getting-started = Guide de démarrage
footer-docs-api-reference = Référence API
footer-docs-examples = Exemples
footer-resources-heading = Ressources
footer-resources-api-v1 = Documentation osu! API V1
footer-resources-api-v2 = Documentation osu! API V2
footer-resources-leptos = Framework Leptos
footer-resources-issues = Signaler un problème
footer-resources-changelog = Journal des modifications
footer-copyright = {$year} Osynicite. OsynicOsuapi est un projet non officiel, sans lien avec osu!.
footer-terms = Conditions d'utilisation
footer-privacy = Politique de confidentialité


# Hero section
hero-badge = CLIENT API OSU! EN RUST
hero-title-highlight = OsynicOsuapi
hero-title-subtext = Client API osu! en Rust haute performance, bien structuré et extensible
hero-description = Client API typé, compatible avec l'asynchrone et WASM, pour intégrer facilement votre projet Rust avec osu!.
hero-button-docs = Voir la documentation
hero-button-demo = Essayer la démo
hero-feature-api = Support des API v1 et v2
hero-feature-type-safe = Sûreté du typage
hero-feature-wasm = Compatible WASM


# API Section
api-section-title = Couverture API complète
api-section-description = OsynicOsuapi offre un accès complet et typé à tous les points de terminaison de l'API osu!, couvrant la majorité des versions v1 et v2.

# V1 API
api-v1-title = API v1

# V1 API Cards
api-beatmaps-title = API des beatmaps
api-beatmaps-description = Interrogation des informations sur les beatmaps, y compris les détails, les scores et le statut. Prend en charge la recherche par ID, ID de set ou hash.
api-users-title = API des utilisateurs
api-users-description = Obtention des profils utilisateurs, statistiques, meilleures performances et parties récentes. Prend en charge la recherche par nom d'utilisateur ou ID.
api-scores-title = API des scores
api-scores-description = Obtention d'informations sur les scores pour différentes beatmaps et utilisateurs. Prend en charge la recherche par mode et filtres.
api-replays-title = API des replays
api-replays-description = Obtention des données de replay des utilisateurs. Prend en charge la recherche par ID.
api-multiplayer-title = API multijoueur
api-multiplayer-description = Obtention d'informations sur les salles multijoueur, les enregistrements de jeu et les données des participants.

# V2 API
api-v2-title = API v2
api-v2-demo-title-1 = Démonstration osu! API V2 (WASM)
api-v2-demo-description-1 = Nous avons fourni l'ID client, l'URI de redirection et l'URL du proxy correspondant à ce site web.
api-v2-demo-description-2 = Vous pouvez directement cliquer sur le bouton d'authentification pour découvrir comment OsynicOsuapi fonctionne dans le navigateur.
api-v2-demo-description-3 = Si vous souhaitez déployer localement et utiliser votre propre client, allez à
api-v2-demo-api-section = et faites défiler vers le bas pour demander un OAuth ci-dessous.
api-v2-demo-description-4 = En raison des restrictions CORS, vous devez configurer un serveur proxy pour transmettre les demandes. Pour les détails d'implémentation et de déploiement, veuillez consulter

# V2 API Cards
api-oauth-title = API OAuth
api-oauth-description = Gestion du flux d'authentification OAuth 2.0.
api-v2-beatmaps-title = API des beatmaps
api-v2-beatmaps-description = Interrogation des informations sur les beatmaps, détails et métadonnées.
api-beatmapsets-title = API des sets de beatmaps
api-beatmapsets-description = Obtention d'informations sur les sets de beatmaps et les beatmaps qu'ils contiennent.
api-changelog-title = API du journal des modifications
api-changelog-description = Obtention des mises à jour et des changements du jeu.
api-chat-title = API de chat
api-chat-description = Obtention de l'historique du chat et envoi de messages.
api-comments-title = API des commentaires
api-comments-description = Obtention et publication de commentaires.
api-events-title = API des événements
api-events-description = Obtention des derniers événements et activités.
api-forum-title = API du forum
api-forum-description = Obtention des sujets et messages du forum.
api-home-title = API de la page d'accueil
api-home-description = Obtention des données de la page d'accueil personnelle.
api-matches-title = API des matchs
api-matches-description = Obtention d'informations sur les matchs officiels.
api-v2-multiplayer-title = API multijoueur
api-v2-multiplayer-description = Obtention et gestion des salles multijoueur.
api-news-title = API des actualités
api-news-description = Obtention des dernières actualités du jeu.
api-notifications-title = API des notifications
api-notifications-description = Obtention des informations de notification personnelles.
api-rankings-title = API des classements
api-rankings-description = Obtention de données détaillées sur les classements.
api-v2-scores-title = API des scores
api-v2-scores-description = Obtention d'informations détaillées sur les scores.
api-v2-users-title = API des utilisateurs
api-v2-users-description = Obtention d'informations détaillées sur les utilisateurs.
api-wiki-title = API wiki
api-wiki-description = Obtention du contenu des pages wiki.
api-friends-title = API des amis
api-friends-description = Obtention et gestion de la liste d'amis.

# Features Section
features-section-title = Pourquoi choisir OsynicOsuapi ?
features-section-description = Une bibliothèque API osu! haute performance et typée, offrant une expérience de développement complète pour diverses plateformes
features-high-performance-title = Haute performance
features-high-performance-description = Construit avec Rust, offrant d'excellentes performances et sécurité mémoire, idéal pour les applications à haut débit. Que ce soit pour des applications serveur ou client, vous obtiendrez les meilleures performances.
features-type-safe-title = Sûreté du typage
features-type-safe-description = Utilise le système de typage fort de Rust pour capturer les erreurs à la compilation et fournit un modèle API complet. Basé sur une division en trois modules - client, interface et modèle - la structure est claire et facile à comprendre.
features-multi-platform-title = Multiplateforme
features-multi-platform-description = Prend en charge les environnements WASM, natifs et serveur, vous permettant d'utiliser la même API partout. Offre un support WebAssembly pour l'interface V1, permettant d'accéder directement à l'API OSU depuis les applications web.
features-api-support-title = Support API complet
features-api-support-description = Prend en charge tous les points de terminaison OSU! V1 et la plupart des points de terminaison V2 (à l'exception des interfaces non catégorisées dans la documentation). Quelle que soit la version API dont votre projet a besoin, l'intégration est simple.
features-flexible-extension-title = Extension flexible
features-flexible-extension-description = La partie client agrège les interfaces et prend en charge plusieurs clients HTTP, facilitant l'extension. La conception modulaire vous permet de personnaliser l'implémentation selon vos besoins et de vous adapter facilement à différents scénarios d'application.
features-rich-examples-title = Exemples riches
features-rich-examples-description = Fournit un support complet d'exemples. Exécutez `cargo run --example nom_exemple` pour voir les données retournées et apprendre rapidement. Apprendre à partir d'exemples de code est le meilleur moyen de maîtriser cette bibliothèque.
features-cta-button = Voir la page OsynicOsuapi sur crates.io
features-cta-button-npm = Voir la page OsynicOsuapi sur npm

# Quick Start Section
quick-start-title = Démarrage rapide
quick-start-description = Configuration simple en quelques étapes pour un accès rapide à l'API osu!
quick-start-reqwest-client = Client Reqwest (Rust)
quick-start-env-var = Variables d'environnement
quick-start-config-file = Fichier de configuration
quick-start-example-code = Code d'exemple
quick-start-wasm-client = Client WASM
quick-start-view-examples = Voir les exemples de code
quick-start-vue-client = Client Vue (Typescript/Javascript)
quick-start-vite-config = Configuration Vite
quick-start-install = Installation des dépendances

# API Demo Section
api-demo-title-1 = Expérience démo de l'API osu! V1 (WASM)
api-demo-description-1 = Entrez votre clé API osu! pour essayer OsynicOsuapi dans le navigateur.
api-demo-description-2 = Si vous n'avez pas de clé API, rendez-vous sur
api-demo-account-settings = votre page de paramètres osu!
api-demo-api-section = et faites défiler jusqu'à la section "API héritée" pour en demander une.
api-demo-label-api-key = Clé API
api-demo-input-placeholder = Entrez votre clé API osu!
api-demo-key-security = La clé n'est utilisée que dans votre navigateur et n'est pas transmise ailleurs.


# API Demo Additional Elements
api-demo-beatmap-query = Recherche de beatmap
api-demo-user-query = Recherche d'utilisateur
api-demo-input-beatmap-label = ID de set de beatmap
api-demo-input-beatmap-placeholder = Entrez un ID de set de beatmap
api-demo-input-user-label = Nom d'utilisateur ou ID
api-demo-input-user-placeholder = Entrez un nom d'utilisateur ou ID
api-demo-loading = Chargement
api-demo-search = Rechercher
api-demo-search-result = Résultat de recherche
api-demo-left-formatted = Gauche : Résultats formatés
api-demo-right-raw = Droite : Données JSON brutes
api-demo-formatted-results = Résultats formatés
api-demo-raw-json = JSON API brut
api-demo-description-wasm = Cette démo utilise OsynicOsuapi via WASM et s'exécute directement dans votre navigateur, sans nécessiter de traitement par un serveur backend.
api-demo-view-github = Voir le projet GitHub
api-demo-current-date = Date et heure actuelles (UTC - format YYYY-MM-DD HH:MM:SS) :
api-demo-current-user = Nom de connexion de l'utilisateur actuel :

# API Demo Code Logic
api-demo-result-placeholder = // Les résultats s'afficheront ici
api-demo-raw-json-placeholder = // Le JSON API brut s'affichera ici
api-demo-enter-api-key = Veuillez entrer la clé API
api-demo-loading-text = Chargement...
api-demo-no-beatmap-found = Aucune beatmap trouvée
api-demo-beatmap-number = --- Beatmap #{$number} ---
api-demo-title = Titre : {$title}
api-demo-artist = Artiste : {$artist}
api-demo-version = Nom de difficulté : {$version}
api-demo-bpm = BPM : {$bpm}
api-demo-stars = Étoiles : {$stars}
api-demo-error = Erreur : {$error}
api-demo-no-user-found = Aucun utilisateur trouvé
api-demo-username = Nom d'utilisateur : {$username}
api-demo-user-id = ID utilisateur : {$id}
api-demo-country = Pays : {$country}
api-demo-pp = PP : {$pp}
api-demo-accuracy = Précision : {$accuracy}%
api-demo-global-rank = Classement mondial : #{$rank}
api-demo-country-rank = Classement national : #{$country_rank}
api-demo-playcount = Nombre de parties : {$count}

# Traductions de démonstration des jeux récents de l'utilisateur
api-demo-no-recent-found = Aucun jeu récent trouvé pour cet utilisateur
api-demo-mode-label = Mode de jeu
api-demo-mode-osu = osu! (Standard)
api-demo-mode-taiko = Taiko
api-demo-mode-ctb = Catch the Beat
api-demo-mode-mania = osu!mania
api-demo-limit-label = Limite de résultats (1-50)
api-demo-limit-placeholder = Entrez le nombre de résultats (par défaut : 10)
api-demo-search-recent = Rechercher les jeux récents

# Affichage des résultats de jeux récents
api-demo-recent-play = Jeu #{$index}
api-demo-beatmap-id = ID de la beatmap : {$id}
api-demo-score = Score : {$score}
api-demo-combo = Combo max : {$combo}x
api-demo-accuracy-hits = Coups : {$c300}×300 / {$c100}×100 / {$c50}×50 / {$miss}×Miss
api-demo-rank = Rang : {$rank}
api-demo-mods = Mods : {$mods}
api-demo-date = Date : {$date}
api-demo-perfect = Combo parfait : {$perfect}
api-demo-user-recent-query = Recherche des jeux récents de l'utilisateur

# Recherche des meilleurs scores de l'utilisateur
api-demo-best-play = Meilleur score de l'utilisateur
api-demo-no-best-found = Aucun meilleur score trouvé pour cet utilisateur
api-demo-user-best-query = Recherche des meilleurs scores de l'utilisateur

# Scores Demo
api-demo-scores-query = Requête de scores
api-demo-no-scores-found = Aucun score trouvé pour cette beatmap
api-demo-score-number = Score #{$number}
api-demo-replay = Replay: {$replay}
api-demo-input-user-label-optional = Nom d'utilisateur (Optionnel)
api-demo-input-user-placeholder-optional = Laisser vide pour obtenir tous les scores

api-docs-label = osu! API Docs