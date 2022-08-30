import { DemandFormValue, DemandItemType } from "./types";
import { v4 as uuid } from 'uuid';
import dayjs from "dayjs";

/**
 * 生成提交表单参数
 * @param info 表单信息
 */
export const createDemandItem = (info: DemandFormValue) => {
    return <DemandItemType>{
        id: uuid().replaceAll('-', ''),
        ...info,
        status: 1,
        publish_date: dayjs(info.publish_date).valueOf(),
        create_date: +new Date(),
        update_date: +new Date(),
    };
};

export const editorDemandItem = (info: DemandFormValue, item: DemandItemType) => {
    return {
        ...item,
        ...info,
        update_date: +new Date(),
        publish_date: dayjs(info.publish_date).valueOf(),
    };
}

export const validUrl = (rule: any, value: any, callback: any) => {
    if (!value) callback();
    if (!/^(http(s?)|):\/\/(.+)$/.test(value)) {
        callback(new Error('不是正确的url地址'));
    }
    callback();
}