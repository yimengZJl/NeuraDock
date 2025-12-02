import { useState, useEffect } from 'react';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { toast } from 'sonner';
import { invoke } from '@tauri-apps/api/core';
import type { NotificationChannelDto } from './NotificationChannelList';

interface NotificationChannelDialogProps {
  open: boolean;
  onClose: (success: boolean) => void;
  channel?: NotificationChannelDto | null;
}

type ChannelType = 'feishu' | 'dingtalk' | 'email';

interface FeishuConfig {
  type: 'feishu';
  webhook_key: string;
}

interface DingTalkConfig {
  type: 'dingtalk';
  webhook_key: string;
  secret?: string;
}

interface EmailConfig {
  type: 'email';
  smtp_host: string;
  smtp_port: number;
  username: string;
  password: string;
  from: string;
  to: string[];
}

type ChannelConfig = FeishuConfig | DingTalkConfig | EmailConfig;

export function NotificationChannelDialog({ open, onClose, channel }: NotificationChannelDialogProps) {
  const [channelType, setChannelType] = useState<ChannelType>('feishu');
  const [webhookKey, setWebhookKey] = useState('');
  const [secret, setSecret] = useState('');
  const [smtpHost, setSmtpHost] = useState('');
  const [smtpPort, setSmtpPort] = useState(465);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [from, setFrom] = useState('');
  const [to, setTo] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (channel) {
      try {
        const config = JSON.parse(channel.config);
        setChannelType(channel.channel_type as ChannelType);

        if (config.type === 'feishu') {
          setWebhookKey(config.webhook_key || '');
        } else if (config.type === 'dingtalk') {
          setWebhookKey(config.webhook_key || '');
          setSecret(config.secret || '');
        } else if (config.type === 'email') {
          setSmtpHost(config.smtp_host || '');
          setSmtpPort(config.smtp_port || 465);
          setUsername(config.username || '');
          setPassword(config.password || '');
          setFrom(config.from || '');
          setTo((config.to || []).join(', '));
        }
      } catch (err) {
        console.error('Failed to parse channel config:', err);
      }
    } else {
      resetForm();
    }
  }, [channel, open]);

  const resetForm = () => {
    setChannelType('feishu');
    setWebhookKey('');
    setSecret('');
    setSmtpHost('');
    setSmtpPort(465);
    setUsername('');
    setPassword('');
    setFrom('');
    setTo('');
  };

  const buildConfig = (): ChannelConfig => {
    if (channelType === 'feishu') {
      return {
        type: 'feishu',
        webhook_key: webhookKey.trim(),
      };
    } else if (channelType === 'dingtalk') {
      return {
        type: 'dingtalk',
        webhook_key: webhookKey.trim(),
        secret: secret.trim() || undefined,
      };
    } else {
      return {
        type: 'email',
        smtp_host: smtpHost.trim(),
        smtp_port: smtpPort,
        username: username.trim(),
        password: password.trim(),
        from: from.trim(),
        to: to.split(',').map(email => email.trim()).filter(Boolean),
      };
    }
  };

  const validateForm = (): boolean => {
    if (channelType === 'feishu') {
      if (!webhookKey.trim()) {
        toast.error('请输入 Webhook Key');
        return false;
      }
    } else if (channelType === 'dingtalk') {
      if (!webhookKey.trim()) {
        toast.error('请输入 Webhook Key');
        return false;
      }
    } else if (channelType === 'email') {
      if (!smtpHost.trim()) {
        toast.error('请输入 SMTP 服务器');
        return false;
      }
      if (!username.trim()) {
        toast.error('请输入用户名');
        return false;
      }
      if (!password.trim()) {
        toast.error('请输入密码');
        return false;
      }
      if (!from.trim()) {
        toast.error('请输入发件人地址');
        return false;
      }
      if (!to.trim()) {
        toast.error('请输入收件人地址');
        return false;
      }
    }
    return true;
  };

  const handleSave = async () => {
    if (!validateForm()) {
      return;
    }

    setSaving(true);
    try {
      const config = buildConfig();

      if (channel) {
        // Update existing channel
        await invoke('update_notification_channel', {
          input: {
            channel_id: channel.id,
            config: config,
            enabled: null,
          },
        });
        toast.success('通知渠道已更新');
      } else {
        // Create new channel
        await invoke('create_notification_channel', {
          input: {
            channel_type: channelType,
            config: config,
          },
        });
        toast.success('通知渠道已创建');
      }

      onClose(true);
    } catch (err) {
      toast.error(channel ? '更新失败' : '创建失败', {
        description: String(err),
      });
    } finally {
      setSaving(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={(isOpen) => !isOpen && onClose(false)}>
      <DialogContent className="sm:max-w-[500px] rounded-2xl">
        <DialogHeader>
          <DialogTitle>{channel ? '编辑通知渠道' : '添加通知渠道'}</DialogTitle>
          <DialogDescription>
            配置通知渠道，签到成功/失败时将自动发送通知
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Channel Type */}
          <div className="space-y-2">
            <Label>渠道类型</Label>
            <Select
              value={channelType}
              onValueChange={(value) => setChannelType(value as ChannelType)}
              disabled={!!channel}
            >
              <SelectTrigger className="rounded-lg">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="feishu">飞书</SelectItem>
                <SelectItem value="dingtalk">钉钉 (即将支持)</SelectItem>
                <SelectItem value="email">邮件 (即将支持)</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {/* Feishu Config */}
          {channelType === 'feishu' && (
            <div className="space-y-2">
              <Label htmlFor="webhook-key">Webhook Key *</Label>
              <Input
                id="webhook-key"
                value={webhookKey}
                onChange={(e) => setWebhookKey(e.target.value)}
                placeholder="从飞书机器人 URL 中提取的 key"
                className="rounded-lg"
              />
              <p className="text-xs text-muted-foreground">
                飞书机器人 URL 格式: https://open.feishu.cn/open-apis/bot/v2/hook/<strong>xxx</strong>
                <br />
                请填写 xxx 部分
              </p>
            </div>
          )}

          {/* DingTalk Config */}
          {channelType === 'dingtalk' && (
            <>
              <div className="space-y-2">
                <Label htmlFor="webhook-key-dt">Webhook Key *</Label>
                <Input
                  id="webhook-key-dt"
                  value={webhookKey}
                  onChange={(e) => setWebhookKey(e.target.value)}
                  placeholder="从钉钉机器人 URL 中提取的 key"
                  className="rounded-lg"
                  disabled
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="secret">加签密钥 (可选)</Label>
                <Input
                  id="secret"
                  value={secret}
                  onChange={(e) => setSecret(e.target.value)}
                  placeholder="加签密钥"
                  className="rounded-lg"
                  disabled
                />
              </div>
              <p className="text-xs text-yellow-600">
                钉钉通知功能即将支持
              </p>
            </>
          )}

          {/* Email Config */}
          {channelType === 'email' && (
            <>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="smtp-host">SMTP 服务器 *</Label>
                  <Input
                    id="smtp-host"
                    value={smtpHost}
                    onChange={(e) => setSmtpHost(e.target.value)}
                    placeholder="smtp.example.com"
                    className="rounded-lg"
                    disabled
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="smtp-port">端口 *</Label>
                  <Input
                    id="smtp-port"
                    type="number"
                    value={smtpPort}
                    onChange={(e) => setSmtpPort(parseInt(e.target.value))}
                    placeholder="465"
                    className="rounded-lg"
                    disabled
                  />
                </div>
              </div>

              <div className="space-y-2">
                <Label htmlFor="username">用户名 *</Label>
                <Input
                  id="username"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  placeholder="your@email.com"
                  className="rounded-lg"
                  disabled
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="password">密码 *</Label>
                <Input
                  id="password"
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="密码或授权码"
                  className="rounded-lg"
                  disabled
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="from">发件人地址 *</Label>
                <Input
                  id="from"
                  value={from}
                  onChange={(e) => setFrom(e.target.value)}
                  placeholder="sender@example.com"
                  className="rounded-lg"
                  disabled
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="to">收件人地址 *</Label>
                <Input
                  id="to"
                  value={to}
                  onChange={(e) => setTo(e.target.value)}
                  placeholder="用逗号分隔多个邮箱地址"
                  className="rounded-lg"
                  disabled
                />
              </div>

              <p className="text-xs text-yellow-600">
                邮件通知功能即将支持
              </p>
            </>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onClose(false)} className="rounded-full">
            取消
          </Button>
          <Button
            onClick={handleSave}
            disabled={saving || (channelType !== 'feishu')}
            className="rounded-full"
          >
            {saving ? '保存中...' : channel ? '更新' : '创建'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
