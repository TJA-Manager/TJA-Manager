<script lang="ts">
    import { onMount } from 'svelte';
    import { getVersion } from '@tauri-apps/api/app';
    import { openUrl } from '@tauri-apps/plugin-opener';
    import { Tabs, Dialog, Portal, Collapsible } from '@skeletonlabs/skeleton-svelte';
    import { AppWindow, List, BookText, XIcon, MinusIcon, PlusIcon, ScrollText, Globe } from 'lucide-svelte';

    import iconUrl from '$lib/images/icon.png'

    let version = $state("Loading...");
    
    const animation = 'transition transition-discrete opacity-0 translate-y-[100px] starting:data-[state=open]:opacity-0 starting:data-[state=open]:translate-y-[100px] data-[state=open]:opacity-100 data-[state=open]:translate-y-0';

    async function getCurrentVersion() {
        const TJAMGRVersion = await getVersion();
        version = TJAMGRVersion;
        console.log("Current version: " + version);
    }

    onMount(async () => {
        getCurrentVersion()
    });
</script>

<main>
    <Tabs defaultValue="1">
        <Tabs.List class="m-1">    
            <Tabs.Trigger value="1" class="text-[12px]"><AppWindow/> About TJAMGR</Tabs.Trigger>
            <Tabs.Trigger value="2" class="text-[12px]"><List/> Changelog</Tabs.Trigger>
            <Tabs.Trigger value="3" class="text-[12px]"><BookText/> Documentation</Tabs.Trigger>

            <Tabs.Indicator class="bg-primary-400"/>
        </Tabs.List>
        
        <Tabs.Content value="1" class="card preset-filled-surface-100-900">
            <article class="flex">
                <img src={iconUrl} alt="TJAMGR icon" class="size-48 mr-3">
                        
                <section class="relative w-full">
                    <h1><b>TJAMGR:</b> A hassle free tool to manage TJAs.</h1>
                    <hr>
                    <p>Current version: {version}</p>
                    <hr>
                    <Dialog closeOnInteractOutside={false}>
                        <Dialog.Trigger class="btn btn-action"><ScrollText/> Credits</Dialog.Trigger>
                        <Portal>
                            <Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
                            <Dialog.Positioner class="fixed inset-0 z-50 grid place-items-center size-screen p-[92px] overflow-y-auto">
                                <Dialog.Content class="card preset-filled-surface-100-900 bg-surface-100-900 w-full {animation}">
                                    <header class="flex justify-between items-center">
                                        <Dialog.Title><h1>Credits</h1></Dialog.Title>
                                        <Dialog.CloseTrigger class="btn-icon btn-ghost"><XIcon/></Dialog.CloseTrigger>
                                    </header>
                                    
                                    <Dialog.Description class="overflow-y-auto">
                                        <hr>
                                        <p>The font used in TJA Manager, and its logo, is <button class="anchor" onclick={() => openUrl("https://fonts.google.com/specimen/Mochiy+Pop+One")}>Mochiy Pop One</button>. Which is licensed under the <button class="anchor" onclick={() => openUrl("https://openfontlicense.org/open-font-license-official-text/")}>OFL Version 1.1</button>.</p>
                                        <blockquote class="blockquote my-2 not-italic">
                                            <small>
                                                Copyright 2020 The Mochiypop Project Authors (<button class="anchor" onclick={() => openUrl("https://github.com/fontdasu/Mochiypop")}>https://github.com/fontdasu/Mochiypop</button>)
                                                <br>This Font Software is licensed under the <button class="anchor" onclick={() => openUrl("https://openfontlicense.org/open-font-license-official-text/")}>SIL Open Font License, Version 1.1</button>. This license is copied below, and is also available with a FAQ at: <button class="anchor" onclick={() => openUrl("https://openfontlicense.org")}>https://openfontlicense.org</button>
                                                <br><br><button class="anchor" onclick={() => openUrl("https://openfontlicense.org/open-font-license-official-text/")}>SIL OPEN FONT LICENSE Version 1.1</button> - 26 February 2007 
                                            </small>
                                        </blockquote>
                                        <Collapsible class="items-start w-full p-1.5 my-1.5 preset-filled-surface-200-800 rounded-container">
                                            <div class="w-full px-0.5 gap-1.5 flex justify-between items-center">
                                                <h6 class="flex items-center gap-1">SIL OPEN FONT LICENSE Version 1.1 <span class="text-primary-500">-</span> <small class="opacity-50">26 February 2007</small></h6>
                                                <Collapsible.Trigger class="btn hover:preset-tonal group"> 
                                                    <MinusIcon class="size-6 group-data-[state=open]:block hidden" />
                                                    <PlusIcon class="size-6 group-data-[state=open]:hidden block" />
                                                </Collapsible.Trigger>
                                            </div>

                                            <Collapsible.Content class="w-full p-2.5 preset-filled-surface-300-700 rounded-container wrap-break-word">
                                                <h3>PREAMBLE</h3>
                                                <hr>
                                                <p>The goals of the Open Font License (OFL) are to stimulate worldwide development of collaborative font projects, to support the font creation efforts of academic and linguistic communities, and to provide a free and open framework in which fonts may be shared and improved in partnership with others.</p>
                                                <br>
                                                <p>The OFL allows the licensed fonts to be used, studied, modified and redistributed freely as long as they are not sold by themselves. The fonts, including any derivative works, can be bundled, embedded, redistributed and/or sold with any software provided that any reserved names are not used by derivative works. The fonts and derivatives, however, cannot be released under any other type of license. The requirement for fonts to remain under this license does not apply to any document created using the fonts or their derivatives.</p>
                                                <br>
                                                <h3>DEFINITIONS</h3>
                                                <hr>
                                                <p><span class="text-primary-500">"Font Software"</span> refers to the set of files released by the Copyright Holder(s) under this license and clearly marked as such. This may include source files, build scripts and documentation.</p>
                                                <br>
                                                <p><span class="text-primary-500">"Reserved Font Name"</span> refers to any names specified as such after the copyright statement(s).</p>
                                                <br>
                                                <p><span class="text-primary-500">"Original Version"</span> refers to the collection of Font Software components as distributed by the Copyright Holder(s).</p>
                                                <br>
                                                <p><span class="text-primary-500">"Modified Version"</span> refers to any derivative made by adding to, deleting, or substituting -- in part or in whole -- any of the components of the Original Version, by changing formats or by porting the Font Software to a new environment.</p>
                                                <br>
                                                <p><span class="text-primary-500">"Author"</span> refers to any designer, engineer, programmer, technical writer or other person who contributed to the Font Software.</p>
                                                <br>
                                                <h3>PERMISSION &amp; CONDITIONS</h3>
                                                <hr>
                                                <p>Permission is hereby granted, free of charge, to any person obtaining a copy of the Font Software, to use, study, copy, merge, embed, modify, redistribute, and sell modified and unmodified copies of the Font Software, subject to the following conditions:</p>
                                                <br>
                                                <p><span class="text-primary-500">1.</span> Neither the Font Software nor any of its individual components, in Original or Modified Versions, may be sold by itself.</p>
                                                <br>
                                                <p><span class="text-primary-500">2.</span> Original or Modified Versions of the Font Software may be bundled, redistributed and/or sold with any software, provided that each copy contains the above copyright notice and this license. These can be included either as stand-alone text files, human-readable headers or in the appropriate machine-readable metadata fields within text or binary files as long as those fields can be easily viewed by the user.</p>
                                                <br>
                                                <p><span class="text-primary-500">3.</span> No Modified Version of the Font Software may use the Reserved Font Name(s) unless explicit written permission is granted by the corresponding Copyright Holder. This restriction only applies to the primary font name as presented to the users.</p>
                                                <br>
                                                <p><span class="text-primary-500">4.</span> The name(s) of the Copyright Holder(s) or the Author(s) of the Font Software shall not be used to promote, endorse or advertise any Modified Version, except to acknowledge the contribution(s) of the Copyright Holder(s) and the Author(s) or with their explicit written permission.</p>
                                                <br>
                                                <p><span class="text-primary-500">5.</span> The Font Software, modified or unmodified, in part or in whole, must be distributed entirely under this license, and must not be distributed under any other license. The requirement for fonts to remain under this license does not apply to any document created using the Font Software.</p>
                                                <br>
                                                <h3>TERMINATION</h3>
                                                <hr>
                                                <p>This license becomes null and void if any of the above conditions are not met.</p>
                                                <br>
                                                <h3>DISCLAIMER</h3>
                                                <hr>
                                                <p>THE FONT SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, INCLUDING ANY GENERAL, SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM OTHER DEALINGS IN THE FONT SOFTWARE.</p>
                                            </Collapsible.Content>
                                        </Collapsible>
                                        <hr>
                                        <p>TJA Manager is build with:</p>
                                        <ul>
                                            <li><button class="anchor" onclick={() => openUrl("https://tauri.app/")}>Tauri <small>(v2)</small></button></li>
                                            <li><button class="anchor" onclick={() => openUrl("https://svelte.dev/")}>Svelte <small>(v5)</small></button></li>
                                            <li><button class="anchor" onclick={() => openUrl("https://www.skeleton.dev/")}>Skeleton UI <small>(v4)</small></button></li> 
                                            <li><button class="anchor" onclick={() => openUrl("https://tailwindcss.com/")}>Tailwind CSS <small>(v4)</small></button></li>
                                        </ul>
                                    </Dialog.Description>
                                </Dialog.Content>
                            </Dialog.Positioner>
                        </Portal>
                    </Dialog>
                    <br><button class="btn btn-action mt-2" disabled><Globe/> TJA Manager website</button>
                    <hr>
                    <p>BeaniCraft <span class="text-primary-400">|</span> 2026</p>
                </section>
            </article>
        </Tabs.Content>

        <Tabs.Content value="2" class="card preset-filled-surface-100-900">
            [placeholder for tab 2]
        </Tabs.Content>

        <Tabs.Content value="3" class="card preset-filled-surface-100-900">
            [placeholder for tab 3]
        </Tabs.Content>
    </Tabs>
</main>

<style>
</style>