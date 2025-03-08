

import { createPinia } from 'pinia';
import Button from './components/Button';

// 创建独立pinia实例避免污染主应用
const pinia = createPinia();

const install = (app: any) => {
  app.use(pinia);
  app.component(Button.name, Button);
};

// 导出store定义供主应用使用
export * from './stores/';

export default {
  install,
  pinia,  // 暴露实例供特殊场景使用
  version: '__VERSION__'
};
