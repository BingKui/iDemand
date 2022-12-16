import { Store } from 'tauri-plugin-store-api';
import { AppSettingValue } from '../common/types';
import { APP_DEFAULT_SETTING } from '../constants/setting';
const iDemandStore = new Store('.settings.dat');

const APP_INFO_KEY = 'iDemand-app-setting';

// 设置app设置信息
export const setAppSettingData = async (info: AppSettingValue) => {
    await iDemandStore.set(APP_INFO_KEY, info);
};

// 获取app设置信息
export const getAppSettingData = async (): Promise<AppSettingValue> => {
    try {
        const val = (await iDemandStore.get(APP_INFO_KEY)) || {};
        return { ...APP_DEFAULT_SETTING, ...(val as AppSettingValue) };
    } catch (error) {
        return APP_DEFAULT_SETTING;
    }
};
