use leptos::*;

#[component]
pub fn FilterAccordion() -> impl IntoView {
    view! {
        <details class="filter-accordion">
            <summary class="flex justify-between items-center cursor-pointer p-3 bg-slate-800 rounded-lg text-cyan-400 font-semibold select-none">
                <span>"⚙️ FILTRO POR TRILHAS MOTIVACIONAIS (3 Selecionadas)"</span>
                <span class="text-xs text-gray-400">"Clique para expandir/recolher ▾"</span>
            </summary>
            
            <div class="filter-content p-4 bg-slate-900 mt-2 rounded-lg border border-slate-700">
                <div class="flex items-center gap-2 mb-3">
                    <input 
                        type="text" 
                        placeholder="🔍 Pesquisar trilha (ex: Foco, Impulsividade)..." 
                        class="search-select-input w-full p-2 bg-slate-950 border border-slate-700 rounded text-sm text-gray-200"
                    />
                    <button class="btn-danger-sm whitespace-nowrap bg-red-900/40 border border-red-500 text-red-300 text-xs px-3 py-2 rounded hover:bg-red-800">
                        "Limpar Filtros"
                    </button>
                </div>

                <div class="chips-scroll-row flex overflow-x-auto gap-2 pb-2">
                    <span class="chip-tag flex items-center gap-2 bg-slate-800 border border-cyan-500 text-gray-200 text-xs px-3 py-1 rounded-full whitespace-nowrap">
                        <span class="w-2 h-2 rounded-full bg-cyan-400"></span>
                        "Foco Cortical Sustentado"
                        <button class="hover:text-red-400 ml-1">"×"</button>
                    </span>
                    <span class="chip-tag flex items-center gap-2 bg-slate-800 border border-green-500 text-gray-200 text-xs px-3 py-1 rounded-full whitespace-nowrap">
                        <span class="w-2 h-2 rounded-full bg-green-400"></span>
                        "Controle de Impulsividade"
                        <button class="hover:text-red-400 ml-1">"×"</button>
                    </span>
                    <span class="chip-tag flex items-center gap-2 bg-slate-800 border border-purple-500 text-gray-200 text-xs px-3 py-1 rounded-full whitespace-nowrap">
                        <span class="w-2 h-2 rounded-full bg-purple-400"></span>
                        "Velocidade de Processamento"
                        <button class="hover:text-red-400 ml-1">"×"</button>
                    </span>
                </div>
            </div>
        </details>
    }
}
