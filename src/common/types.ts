// 额外链接
export interface ExtraLinkItem {
    text: string;
    link: string;
}

export interface DemandFormValue {
    name: string;
    desc: string;
    publish_date: number;
    demand_link: string;
    ui_link: string;
    api_link: string;
    code_path: string;
    publish_link: string;
    extra_links: ExtraLinkItem[];
}

export interface DemandItemType {
    id: string;
    name: string;
    desc: string;
    status: number;
    publish_date: number;
    demand_link: string;
    ui_link: string;
    api_link: string;
    code_path: string;
    publish_link: string;
    extra_links: ExtraLinkItem[],
    create_date: number;
    update_date: number;
}

export interface WinInfoValue {
    demand_id: string;
    theme: string;
    x: number;
    y: number;
    fixed: boolean;
    date: number;
}

export interface CodeAppItem {
    text: string;
    icon: string;
    value: string;
    shell: string;
    openUrl: Function;
}

export interface AppSettingValue {
    codeApp: string;
}