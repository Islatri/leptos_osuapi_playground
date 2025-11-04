<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { OsynicOsuApiV2GlooClient } from '@osynicite/osynic-osuapi';
import type { OToken, User, Mode } from '@osynicite/osynic-osuapi';
import { 
  LogIn, 
  LogOut, 
  User as UserIcon, 
  RefreshCw, 
  Settings, 
  Trophy, 
  Target,
  Calendar,
  Award,
  TrendingUp,
  Flame,
  Globe,
  Heart,
  Star,
  ExternalLink,
  Badge,
  Users,
  Link2,
  MessageCircle,
  MapPin,
  Briefcase,
  Sparkles,
  Zap,
  Hash,
  Disc3
} from 'lucide-vue-next';

// LocalStorage key
const TOKEN_STORAGE_KEY = 'osu_api_token';

// Client configuration
const clientId = ref<string>('');
const clientSecret = ref<string>('');
const redirectUri = ref<string>('http://localhost:4000/callback');
const proxyUrl = ref<string>('http://localhost:8000/');

// State
const token = ref<OToken | null>(null);
const client = ref<OsynicOsuApiV2GlooClient | null>(null);
const userData = ref<User | null>(null);
const selectedMode = ref<Mode>('osu');
const loading = ref(false);
const error = ref<string>('');
const activeTab = ref<'config' | 'user'>('config');

// Computed
const isAuthenticated = computed(() => !!token.value);
const authUrl = computed(() => {
  if (!clientId.value || !redirectUri.value) return '';
  const scopes = ['public', 'identify'].join(' ');
  return `https://osu.ppy.sh/oauth/authorize?client_id=${clientId.value}&redirect_uri=${encodeURIComponent(redirectUri.value)}&response_type=code&scope=${encodeURIComponent(scopes)}`;
});

const currentStats = computed(() => {
  if (!userData.value?.statistics_rulesets) return null;
  return userData.value.statistics_rulesets[selectedMode.value];
});

// Token storage helpers
interface StoredToken extends OToken {
  stored_at: number;
}

const saveTokenToStorage = (tokenData: OToken) => {
  const storedToken: StoredToken = {
    ...tokenData,
    stored_at: Date.now()
  };
  localStorage.setItem(TOKEN_STORAGE_KEY, JSON.stringify(storedToken));
};

const loadTokenFromStorage = (): OToken | null => {
  const stored = localStorage.getItem(TOKEN_STORAGE_KEY);
  if (!stored) return null;

  try {
    const storedToken: StoredToken = JSON.parse(stored);
    const now = Date.now();
    const elapsed = Math.floor((now - storedToken.stored_at) / 1000);
    
    if (elapsed >= storedToken.expires_in) {
      localStorage.removeItem(TOKEN_STORAGE_KEY);
      return null;
    }

    return {
      access_token: storedToken.access_token,
      refresh_token: storedToken.refresh_token,
      expires_in: storedToken.expires_in - elapsed,
      token_type: storedToken.token_type
    };
  } catch (e) {
    console.error('Failed to parse stored token:', e);
    localStorage.removeItem(TOKEN_STORAGE_KEY);
    return null;
  }
};

const clearStoredToken = () => {
  localStorage.removeItem(TOKEN_STORAGE_KEY);
};

// Parse OAuth callback from URL hash
const parseOAuthCallback = (): OToken | null => {
  const hash = globalThis.location.hash.substring(1);
  if (!hash) return null;

  const params = new URLSearchParams(hash);
  const accessToken = params.get('access_token');
  const refreshToken = params.get('refresh_token');
  const expiresIn = params.get('expires_in');
  const tokenType = params.get('token_type');

  if (!accessToken || !expiresIn || !tokenType) {
    return null;
  }

  return {
    access_token: accessToken,
    refresh_token: refreshToken || undefined,
    expires_in: Number.parseInt(expiresIn, 10),
    token_type: tokenType
  };
};

// Initialize client with token
const initializeClient = (tokenData: OToken) => {
  token.value = tokenData;
  client.value = new OsynicOsuApiV2GlooClient(tokenData);
  
  if (proxyUrl.value) {
    client.value.setProxyUrl(proxyUrl.value);
  }
};

// Methods
const openAuthUrl = () => {
  if (authUrl.value) {
    window.open(authUrl.value, '_blank');
  }
};

const fetchUserData = async () => {
  if (!client.value) return;

  loading.value = true;
  error.value = '';

  try {
    const data = await client.value.getOwnData(selectedMode.value);
    userData.value = data;
  } catch (e: any) {
    error.value = e.message || 'Failed to fetch user data';
  } finally {
    loading.value = false;
  }
};

const refreshToken = async () => {
  if (!client.value || !clientId.value || !clientSecret.value) return;

  loading.value = true;
  error.value = '';

  try {
    const newToken = await client.value.refreshToken(
      BigInt(clientId.value),
      clientSecret.value,
      undefined
    );

    token.value = newToken;
    client.value = new OsynicOsuApiV2GlooClient(newToken);
    
    if (proxyUrl.value) {
      client.value.setProxyUrl(proxyUrl.value);
    }
  } catch (e: any) {
    error.value = e.message || 'Failed to refresh token';
  } finally {
    loading.value = false;
  }
};

const logout = async () => {
  if (client.value) {
    try {
      await client.value.revokeCurrentToken();
    } catch (e) {
      console.error('Failed to revoke token:', e);
    }
  }

  token.value = null;
  client.value = null;
  userData.value = null;
  clearStoredToken();
  activeTab.value = 'config';
};

// Initialize on mount
onMounted(() => {
  const callbackToken = parseOAuthCallback();
  if (callbackToken) {
    globalThis.location.hash = '';
    saveTokenToStorage(callbackToken);
    initializeClient(callbackToken);
    activeTab.value = 'user';
    fetchUserData();
    return;
  }

  const storedToken = loadTokenFromStorage();
  if (storedToken) {
    initializeClient(storedToken);
    activeTab.value = 'user';
    fetchUserData();
  }
});

// Format helpers
const formatNumber = (num: number | null | undefined): string => {
  if (num === null || num === undefined) return 'N/A';
  return num.toLocaleString();
};

const formatDate = (date: string | null | undefined): string => {
  if (!date) return 'N/A';
  return new Date(date).toLocaleDateString();
};

const formatPlaytime = (seconds: number | null | undefined): string => {
  if (!seconds) return 'N/A';
  const hours = Math.floor(seconds / 3600);
  return `${hours.toLocaleString()}h`;
};

const getModeIcon = (mode: Mode) => {
  const icons: Record<Mode, string> = {
    osu: '○',
    taiko: '◎',
    fruits: '◇',
    mania: '◈'
  };
  return icons[mode];
};

const getPlaystyleIcon = (style: string) => {
  const icons: Record<string, string> = {
    mouse: '🖱',
    keyboard: '⌨',
    tablet: '✏',
    touch: '👆'
  };
  return icons[style] || '❓';
};
</script>

<template>
  <div class="min-h-screen bg-linear-to-br from-slate-950 via-slate-900 to-slate-950 text-slate-100 overflow-x-hidden">
    <!-- Background accent -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <div class="absolute top-0 right-0 w-96 h-96 bg-cyan-500/5 rounded-full blur-3xl"></div>
      <div class="absolute bottom-0 left-0 w-96 h-96 bg-cyan-500/5 rounded-full blur-3xl"></div>
    </div>

    <div class="relative z-10">
      <!-- Header -->
      <div class="border-b border-slate-800/50 bg-slate-900/50 backdrop-blur-sm sticky top-0 z-40">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 bg-linear-to-br from-cyan-400 to-cyan-600 rounded-lg flex items-center justify-center">
                <Star :size="20" className="text-white" fill="white" />
              </div>
              <div>
                <h1 class="text-xl font-bold text-white">osu! API Tester</h1>
                <p class="text-xs text-slate-400">OAuth & Statistics Viewer</p>
              </div>
            </div>
            <div v-if="isAuthenticated" class="flex items-center gap-2">
              <div class="px-3 py-1.5 bg-slate-800 border border-slate-700 rounded text-xs text-slate-300">
                {{ userData?.username }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        <!-- Error Alert -->
        <div v-if="error" class="mb-6 p-4 bg-red-950/30 border border-red-900/50 rounded-lg text-sm text-red-300">
          {{ error }}
        </div>

        <!-- Tabs -->
        <div class="flex gap-1 mb-6 border-b border-slate-800">
          <button
            @click="activeTab = 'config'"
            :class="[
              'px-4 py-3 text-sm font-medium border-b-2 transition-all',
              activeTab === 'config'
                ? 'border-cyan-500 text-cyan-400'
                : 'border-transparent text-slate-400 hover:text-slate-300'
            ]"
          >
            <div class="flex items-center gap-2">
              <Settings :size="16" />
              Configuration
            </div>
          </button>
          <button
            @click="activeTab = 'user'"
            :disabled="!isAuthenticated"
            :class="[
              'px-4 py-3 text-sm font-medium border-b-2 transition-all disabled:opacity-50 disabled:cursor-not-allowed',
              activeTab === 'user'
                ? 'border-cyan-500 text-cyan-400'
                : 'border-transparent text-slate-400 hover:text-slate-300'
            ]"
          >
            <div class="flex items-center gap-2">
              <UserIcon :size="16" />
              User Data
            </div>
          </button>
        </div>

        <!-- Configuration Tab -->
        <div v-show="activeTab === 'config'" class="space-y-6">
          <div class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
            <h2 class="text-lg font-semibold text-white mb-6 flex items-center gap-3">
              <LogIn :size="20" class="text-cyan-400" />
              OAuth Configuration
            </h2>

            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-slate-300 mb-2">Client ID</label>
                <input
                  v-model="clientId"
                  type="text"
                  placeholder="Enter your client ID"
                  class="w-full px-4 py-2.5 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:border-cyan-500 focus:outline-none transition-all"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-300 mb-2">Client Secret</label>
                <input
                  v-model="clientSecret"
                  type="password"
                  placeholder="Enter your client secret"
                  class="w-full px-4 py-2.5 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:border-cyan-500 focus:outline-none transition-all"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-300 mb-2">Redirect URI</label>
                <input
                  v-model="redirectUri"
                  type="text"
                  placeholder="http://localhost:5173/callback"
                  class="w-full px-4 py-2.5 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:border-cyan-500 focus:outline-none transition-all"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-300 mb-2">Proxy URL <span class="text-slate-500">(Optional)</span></label>
                <input
                  v-model="proxyUrl"
                  type="text"
                  placeholder="https://your-proxy.com"
                  class="w-full px-4 py-2.5 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:border-cyan-500 focus:outline-none transition-all"
                />
              </div>
            </div>

            <div class="mt-8">
              <button
                @click="openAuthUrl"
                :disabled="!authUrl"
                class="w-full py-3 bg-linear-to-r from-cyan-500 to-cyan-600 hover:from-cyan-600 hover:to-cyan-700 disabled:from-slate-600 disabled:to-slate-600 text-white font-medium rounded-lg transition-all flex items-center justify-center gap-2 disabled:cursor-not-allowed"
              >
                <ExternalLink :size="18" />
                Authorize with osu!
              </button>
              <p class="mt-3 text-xs text-slate-400 text-center">Click to authenticate. You'll be redirected back automatically.</p>
            </div>
          </div>
        </div>

        <!-- User Data Tab -->
        <div v-show="activeTab === 'user'" class="space-y-6">
          <!-- Mode Selector & Auth Status -->
          <div class="flex items-center justify-between gap-4">
            <div class="flex gap-2">
              <button
                v-for="mode in (['osu', 'taiko', 'fruits', 'mania'] as Mode[])"
                :key="mode"
                @click="selectedMode = mode; fetchUserData()"
                :class="[
                  'px-4 py-2 rounded-lg text-sm font-medium transition-all',
                  selectedMode === mode
                    ? 'bg-cyan-600 text-white'
                    : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
                ]"
              >
                {{ getModeIcon(mode) }} {{ mode }}
              </button>
            </div>
            <div class="flex gap-2">
              <button
                @click="refreshToken"
                :disabled="loading"
                class="px-4 py-2 bg-slate-800 hover:bg-slate-700 disabled:bg-slate-700 text-slate-300 text-sm rounded-lg transition-all flex items-center gap-2"
              >
                <RefreshCw :size="16" :class="{ 'animate-spin': loading }" />
                Refresh
              </button>
              <button
                @click="logout"
                class="px-4 py-2 bg-red-900/30 hover:bg-red-900/50 text-red-300 text-sm rounded-lg transition-all flex items-center gap-2"
              >
                <LogOut :size="16" />
                Logout
              </button>
            </div>
          </div>

          <!-- User Profile Card -->
          <div v-if="userData && currentStats" class="space-y-6">
            <!-- Profile Header -->
            <div class="bg-slate-800/50 border border-slate-700/50 rounded-lg overflow-hidden">
              <!-- Cover -->
              <div 
                class="h-32 bg-linear-to-r from-cyan-500/20 to-cyan-400/20 relative"
                :style="userData.cover_url ? `background-image: url(${userData.cover_url}); background-size: cover; background-position: center;` : ''"
              >
                <div class="absolute inset-0 bg-linear-to-t from-slate-900 via-transparent to-transparent"></div>
              </div>

              <!-- Profile Info -->
              <div class="px-6 py-6">
                <div class="flex gap-6 -mt-20 mb-6">
                  <img
                    :src="userData.avatar_url"
                    :alt="userData.username"
                    class="w-28 h-28 rounded-lg border-4 border-slate-800 shadow-2xl z-20"
                  />
                  <div class="flex-1 pt-12">
                    <div class="flex items-start justify-between">
                      <div>
                        <h2 class="text-3xl font-bold text-white mb-1">{{ userData.username }}</h2>
                        <div class="flex flex-wrap items-center gap-3 text-sm text-slate-400">
                          <div class="flex items-center gap-1.5">
                            <Globe :size="14" />
                            {{ userData.country?.name || 'Unknown' }}
                          </div>
                          <div class="flex items-center gap-1.5">
                            <Calendar :size="14" />
                            Joined {{ formatDate(userData.join_date) }}
                          </div>
                          <div v-if="userData.is_supporter" class="flex items-center gap-1.5 px-2.5 py-1 bg-pink-900/30 text-pink-300 rounded text-xs font-medium">
                            <Heart :size="12" fill="currentColor" />
                            Supporter
                          </div>
                        </div>
                      </div>
                      <div v-if="currentStats.global_rank" class="text-right">
                        <div class="text-xs text-slate-400 mb-1">Global Rank</div>
                        <div class="text-3xl font-bold text-cyan-400">#{{ formatNumber(currentStats.global_rank) }}</div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Quick Stats -->
                <div class="grid grid-cols-4 gap-4">
                  <div class="bg-slate-900/50 border border-slate-700/30 rounded p-3">
                    <div class="text-xs text-slate-400 mb-1">Performance</div>
                    <div class="text-2xl font-bold text-cyan-400">{{ formatNumber(currentStats.pp) }}pp</div>
                  </div>
                  <div class="bg-slate-900/50 border border-slate-700/30 rounded p-3">
                    <div class="text-xs text-slate-400 mb-1">Accuracy</div>
                    <div class="text-2xl font-bold text-cyan-400">{{ currentStats.hit_accuracy.toFixed(2) }}%</div>
                  </div>
                  <div class="bg-slate-900/50 border border-slate-700/30 rounded p-3">
                    <div class="text-xs text-slate-400 mb-1">Play Count</div>
                    <div class="text-2xl font-bold text-cyan-400">{{ formatNumber(currentStats.play_count) }}</div>
                  </div>
                  <div class="bg-slate-900/50 border border-slate-700/30 rounded p-3">
                    <div class="text-xs text-slate-400 mb-1">Play Time</div>
                    <div class="text-2xl font-bold text-cyan-400">{{ formatPlaytime(currentStats.play_time) }}</div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Rank History -->
            <div v-if="userData.rank_highest" class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
              <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                <TrendingUp :size="16" class="text-cyan-400" />
                Rank History
              </h3>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <div class="text-xs text-slate-400 mb-1">Highest Rank</div>
                  <div class="text-2xl font-bold text-cyan-400">#{{ formatNumber(userData.rank_highest.rank) }}</div>
                  <div class="text-xs text-slate-500 mt-1">Achieved on {{ formatDate(userData.rank_highest.updated_at) }}</div>
                </div>
                <div>
                  <div class="text-xs text-slate-400 mb-1">Current Rank</div>
                  <div class="text-2xl font-bold text-cyan-400">#{{ formatNumber(currentStats.global_rank) }}</div>
                </div>
              </div>
            </div>

            <!-- Stats Grid -->
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <!-- Rankings -->
              <div class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
                <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                  <Trophy :size="16" class="text-cyan-400" />
                  Rankings
                </h3>
                <div class="space-y-3">
                  <div class="flex justify-between items-center">
                    <span class="text-xs text-slate-400">Global Rank</span>
                    <span class="text-lg font-bold text-cyan-400">#{{ formatNumber(currentStats.global_rank) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-xs text-slate-400">Country Rank</span>
                    <span class="text-lg font-bold text-cyan-400">#{{ formatNumber(currentStats.country_rank) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-xs text-slate-400">Level</span>
                    <span class="text-lg font-bold text-cyan-400">{{ currentStats.level.current }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-xs text-slate-400">Level Progress</span>
                    <span class="text-lg font-bold text-cyan-400">{{ currentStats.level.progress }}%</span>
                  </div>
                </div>
              </div>

              <!-- Grade Distribution -->
              <div class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
                <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                  <Award :size="16" class="text-cyan-400" />
                  Grades
                </h3>
                <div class="space-y-2">
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-yellow-300 font-medium">SSH</span>
                    <span class="text-slate-300">{{ formatNumber(currentStats.grade_counts.ssh) }}</span>
                  </div>
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-yellow-300 font-medium">SS</span>
                    <span class="text-slate-300">{{ formatNumber(currentStats.grade_counts.ss) }}</span>
                  </div>
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-slate-300">SH</span>
                    <span class="text-slate-400">{{ formatNumber(currentStats.grade_counts.sh) }}</span>
                  </div>
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-slate-300">S</span>
                    <span class="text-slate-400">{{ formatNumber(currentStats.grade_counts.s) }}</span>
                  </div>
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-green-400 font-medium">A</span>
                    <span class="text-slate-300">{{ formatNumber(currentStats.grade_counts.a) }}</span>
                  </div>
                </div>
              </div>

              <!-- Hit Details -->
              <div class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
                <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                  <Target :size="16" class="text-cyan-400" />
                  Hit Distribution
                </h3>
                <div class="space-y-2">
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-cyan-300">300</span>
                    <span class="text-slate-300">{{ formatNumber(currentStats.count_300) }}</span>
                  </div>
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-green-300">100</span>
                    <span class="text-slate-300">{{ formatNumber(currentStats.count_100) }}</span>
                  </div>
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-yellow-300">50</span>
                    <span class="text-slate-300">{{ formatNumber(currentStats.count_50) }}</span>
                  </div>
                  <div class="flex justify-between items-center text-sm">
                    <span class="text-red-300">Miss</span>
                    <span class="text-slate-300">{{ formatNumber(currentStats.count_miss) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Detailed Stats -->
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
                <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                  <Flame :size="16" class="text-cyan-400" />
                  Score Statistics
                </h3>
                <div class="space-y-3">
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Ranked Score</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(currentStats.ranked_score) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Total Score</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(currentStats.total_score) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Total Hits</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(currentStats.total_hits) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Max Combo</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(currentStats.maximum_combo) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Replays Watched</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(currentStats.replays_watched_by_others) }}</span>
                  </div>
                </div>
              </div>

              <div class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
                <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                  <User class="w-4 h-4" />
                  User Information
                </h3>
                <div class="space-y-3">
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Follower Count</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(userData.follower_count) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Post Count</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(userData.post_count) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Comments Count</span>
                    <span class="font-mono text-slate-200">{{ formatNumber(userData.comments_count) }}</span>
                  </div>
                  <div class="flex justify-between items-center">
                    <span class="text-slate-400 text-sm">Account Status</span>
                    <span class="inline-block px-2 py-1 text-xs rounded" :class="userData.is_active ? 'bg-green-900/30 text-green-300' : 'bg-red-900/30 text-red-300'">
                      {{ userData.is_active ? 'Active' : 'Inactive' }}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Personal Info -->
            <div v-if="userData.location || userData.interests || userData.occupation || userData.website || userData.twitter || userData.discord" class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
              <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                <Sparkles :size="16" class="text-cyan-400" />
                Personal Profile
              </h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div v-if="userData.location" class="flex gap-3">
                  <MapPin :size="16" class="text-slate-500 shrink-0 mt-0.5" />
                  <div>
                    <div class="text-xs text-slate-400">Location</div>
                    <div class="text-slate-200">{{ userData.location }}</div>
                  </div>
                </div>
                <div v-if="userData.occupation" class="flex gap-3">
                  <Briefcase :size="16" class="text-slate-500 shrink-0 mt-0.5" />
                  <div>
                    <div class="text-xs text-slate-400">Occupation</div>
                    <div class="text-slate-200">{{ userData.occupation }}</div>
                  </div>
                </div>
                <div v-if="userData.interests" class="flex gap-3 md:col-span-2">
                  <Sparkles :size="16" class="text-slate-500 shrink-0 mt-0.5" />
                  <div>
                    <div class="text-xs text-slate-400">Interests</div>
                    <div class="text-slate-200">{{ userData.interests }}</div>
                  </div>
                </div>
                <div v-if="userData.website" class="flex gap-3">
                  <Link2 :size="16" class="text-slate-500 shrink-0 mt-0.5" />
                  <div>
                    <div class="text-xs text-slate-400">Website</div>
                    <a :href="userData.website" target="_blank" class="text-cyan-400 hover:text-cyan-300 break-all">
                      {{ userData.website }}
                    </a>
                  </div>
                </div>
                <div v-if="userData.twitter" class="flex gap-3">
                  <MessageCircle :size="16" class="text-slate-500 shrink-0 mt-0.5" />
                  <div>
                    <div class="text-xs text-slate-400">Twitter</div>
                    <a :href="`https://twitter.com/${userData.twitter}`" target="_blank" class="text-cyan-400 hover:text-cyan-300">
                      @{{ userData.twitter }}
                    </a>
                  </div>
                </div>
                <div v-if="userData.discord" class="flex gap-3">
                  <Disc3 :size="16" class="text-slate-500 shrink-0 mt-0.5" />
                  <div>
                    <div class="text-xs text-slate-400">Discord</div>
                    <div class="text-slate-200">{{ userData.discord }}</div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Playstyle & Groups -->
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div v-if="userData.playstyle && userData.playstyle.length > 0" class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
                <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                  <Zap :size="16" class="text-cyan-400" />
                  Playstyle
                </h3>
                <div class="flex flex-wrap gap-2">
                  <span v-for="style in userData.playstyle" :key="style" class="px-3 py-1.5 bg-slate-900 border border-slate-700 rounded text-sm text-slate-300">
                    {{ getPlaystyleIcon(style) }} {{ style }}
                  </span>
                </div>
              </div>

              <div v-if="userData.groups && userData.groups.length > 0" class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
                <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                  <Users :size="16" class="text-cyan-400" />
                  Groups
                </h3>
                <div class="space-y-2">
                  <div v-for="group in userData.groups" :key="group.id" class="flex items-center justify-between text-sm">
                    <span class="text-slate-300">{{ group.name }}</span>
                    <span class="text-xs text-slate-500">{{ group.short_name }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Badges & Achievements -->
            <div v-if="userData.user_achievements && userData.user_achievements.length > 0" class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
              <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                <Badge :size="16" class="text-cyan-400" />
                Achievements ({{ userData.user_achievements.length }})
              </h3>
              <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-3">
                <div v-for="achievement in userData.user_achievements" :key="achievement.achievement_id" class="flex flex-col items-center gap-2">
                  <div class="w-12 h-12 bg-slate-900 border border-slate-700 rounded-lg flex items-center justify-center">
                    <Sparkles :size="20" class="text-yellow-400" />
                  </div>
                  <div class="text-xs text-slate-400 text-center">
                    #{{ achievement.achievement_id }}
                  </div>
                  <div class="text-xs text-slate-500 text-center">
                    {{ formatDate(achievement.achieved_at) }}
                  </div>
                </div>
              </div>
            </div>

            <!-- Account History -->
            <div v-if="userData.previous_usernames && userData.previous_usernames.length > 0" class="bg-slate-800/50 border border-slate-700/50 rounded-lg p-6">
              <h3 class="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                <Hash :size="16" class="text-cyan-400" />
                Previous Usernames
              </h3>
              <div class="flex flex-wrap gap-2">
                <span v-for="(name, idx) in userData.previous_usernames" :key="idx" class="px-3 py-1.5 bg-slate-900 border border-slate-700 rounded text-sm text-slate-400">
                  {{ name }}
                </span>
              </div>
            </div>
          </div>

          <!-- Loading State -->
          <div v-else-if="loading" class="flex flex-col items-center justify-center py-20 gap-3">
            <RefreshCw class="w-12 h-12 text-cyan-400 animate-spin" />
            <p class="text-slate-400">Loading user data...</p>
          </div>

          <!-- Empty State -->
          <div v-else class="text-center py-20">
            <UserIcon class="w-16 h-16 text-slate-600 mx-auto mb-4" />
            <p class="text-slate-400">No user data available. Please authenticate first.</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>